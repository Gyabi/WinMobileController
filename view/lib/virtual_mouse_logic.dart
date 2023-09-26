import 'dart:async';
import 'dart:ffi';

import 'package:flutter/material.dart';
import 'package:logging/logging.dart';
import "package:gesture_x_detector/gesture_x_detector.dart";

final _logger = Logger('VirtualMouseLogic');


class VirtualMouseLogic extends ChangeNotifier{
  // 送信間隔[ms]
  final int _duration = 100;
  // 接続状態
  bool _isConnected = false;
  // 各種処理に対応するデータ保存用変数
  bool _isPushLeftButton = false;
  bool _isPushWheelButton = false;
  bool _isPushRightButton = false;
  double _scrollWheelDelta = 0.0;
  List<double> _zoomDeltaList = [];
  Offset _mouseMoveDelta = Offset(0.0, 0.0);

  // constructor
  VirtualMouseLogic() {
    // 一定間隔でデータ送信処理を実行
    Timer.periodic(Duration(milliseconds: _duration), (timer) {
      update();
    });
  }

  // mqtt接続
  bool createConnection() {
    _isConnected = true;
    notifyListeners();
    _logger.info("create connection");
    // 接続成功したらtrueを返す
    return true;
  }

  // mqtt接続解除
  bool deleteConnection() {
    _isConnected = false;
    notifyListeners();
    _logger.info("delete connection");
    // 接続解除成功したらtrueを返す
    return true;
  }

  // mqtt接続確認
  bool isConnected() {
    return _isConnected;
  }

  // データ送信
  bool sendData() {
    // データ送信成功したらtrueを返す
    return true;
  }

  void onPushMouseLeftButton() {
    _isPushLeftButton = true;
  }

  void onPushMouseWheelButton() {
    _isPushWheelButton = true;
  }

  void onStartScrollMouseWheel(MoveEvent event) {
    // 開始タイミングは無視
  }

  void onUpdateScrollMouseWheel(MoveEvent event) {
    _scrollWheelDelta += event.delta.dy;
  }

  void onEndScrollMouseWheel(MoveEvent event) {
    // 終了タイミングは無視
  }

  void onPushMouseRightButton() {
    _isPushRightButton = true;
  }

  void onStartZoom(Offset event) {
    // 開始タイミングは無視
  }

  void onUpdateZoom(ScaleEvent event) {
    // 前回からの差分を変化量に加算
    _zoomDeltaList.add(event.scale);
  }

  void onEndZoom() {
    // 終了タイミングは無視
  }

  void onStartMouseMove(MoveEvent event) {
    // 開始タイミングは無視
  }

  void onUpdateMouseMove(MoveEvent event) {
    // 前回からの差分を変化量に加算
    _mouseMoveDelta += event.delta;
  }

  void onEndMouseMove(MoveEvent event) {
    // 終了タイミングは無視
  }

  void update() {
    // 接続されていなければ何もしない
    if(_isConnected == false) {
      return;
    }

    // データがたまっていれば送信処理へ移行
    // TODO:送信時の値をのレンジはあとで調整
    if(_isPushLeftButton) {
      _isPushLeftButton = false;
      _logger.info("send left button");
    }

    if(_isPushWheelButton) {
      _isPushWheelButton = false;
      _logger.info("send wheel button");
    }

    if(_isPushRightButton) {
      _isPushRightButton = false;
      _logger.info("send right button");
    }

    if(_scrollWheelDelta != 0.0) {
      _logger.info("send scroll wheel delta: $_scrollWheelDelta");
      _scrollWheelDelta = 0.0;
    }

    if(_zoomDeltaList.isNotEmpty) {
      // リストの最後の要素から最初の要素を引いた値を送信
      double delta = _zoomDeltaList.last - _zoomDeltaList.first;
      _logger.info("send zoom delta: $delta");
      _zoomDeltaList.clear();
    }

    if(_mouseMoveDelta.dx != 0.0 || _mouseMoveDelta.dy != 0.0) {
      _logger.info("send mouse move delta: $_mouseMoveDelta");
      _mouseMoveDelta = const Offset(0.0, 0.0);
    }
  }
}