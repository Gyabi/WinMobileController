import 'dart:async';
import "dart:convert";

import 'package:flutter/material.dart';
import 'package:logging/logging.dart';
import "package:gesture_x_detector/gesture_x_detector.dart";
import 'package:shared_preferences/shared_preferences.dart';
import 'mqtt_publisher.dart';

final _logger = Logger('VirtualMouseLogic');


class VirtualMouseLogic extends ChangeNotifier{
  // MQTTパブリッシャー
  final MQTTPublisher _publisher = MQTTPublisher();
  // 送信間隔[ms]
  final int _duration = 100;
  // 各種処理に対応するデータ保存用変数
  bool _isPushLeftButton = false;
  bool _isPushWheelButton = false;
  bool _isPushRightButton = false;
  double _scrollWheelDelta = 0.0;
  List<double> _zoomDeltaList = [];
  Offset _mouseMoveDelta = Offset(0.0, 0.0);

  // 各種設定値
  // IPアドレス
  String _ipAddress = '';
  // ポート番号
  int _port = 0;
  // ホイール感度
  double _wheelSensitivity = 1.0;
  // ズーム感度
  double _zoomSensitivity = 1.0;
  // マウス感度
  double _mouseSensitivity = 1.0;

  // constructor
  VirtualMouseLogic() {
    // 初期化処理
    readParams();
    // 一定間隔でデータ送信処理を実行
    Timer.periodic(Duration(milliseconds: _duration), (timer) {
      update();
    });
  }

  // 初期化処理
  Future readParams() async {
    SharedPreferences pref = await SharedPreferences.getInstance();
    // IPアドレスを取得
    _ipAddress = pref.getString('ipAddress') ?? '';
    // ポート番号を取得
    _port = pref.getInt('port') ?? 0; 
    // ホイール感度を取得
    _wheelSensitivity = pref.getDouble('wheelSensitivity') ?? 1.0;
    // ズーム感度を取得
    _zoomSensitivity = pref.getDouble('zoomSensitivity') ?? 1.0;
    // マウス感度を取得
    _mouseSensitivity = pref.getDouble('mouseSensitivity') ?? 1.0;
  }

  // mqtt接続
  Future<bool> createConnection() async {
    bool result = await _publisher.createConnection(_ipAddress, _port);
    if(result) {
      notifyListeners();
    }
    return result;
  }

  // mqtt接続解除
  bool deleteConnection() {
    bool result = _publisher.deleteConnection();
    if(result) {
      notifyListeners();
    }
    return result;
  }

  // mqtt接続確認
  bool isConnected() {
    return _publisher.isConnected();
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
    if(isConnected() == false) {
      return;
    }

    // データがたまっていれば送信処理へ移行
    if(_isPushLeftButton) {
      _isPushLeftButton = false;
      _logger.info("send left button");
      String payload = json.encode({
        'button': "Left"
      }); 
      bool result = _publisher.sendData('WinMobControl/PushMouseButton', payload);
      if(result == false) {
        _logger.info("send left button failed");
      }
    }

    if(_isPushWheelButton) {
      _isPushWheelButton = false;
      _logger.info("send wheel button");
      String payload = json.encode({
        'button': "Wheel"
      });
      bool result = _publisher.sendData('WinMobControl/PushMouseButton', payload);
      if(result == false) {
        _logger.info("send wheel button failed");
      }
    }

    if(_isPushRightButton) {
      _isPushRightButton = false;
      _logger.info("send right button");
      String payload = json.encode({
        'button': "Right"
      });
      bool result = _publisher.sendData('WinMobControl/PushMouseButton', payload);
      if(result == false) {
        _logger.info("send right button failed");
      }
    }

    if(_scrollWheelDelta != 0.0) {
      _scrollWheelDelta *= _wheelSensitivity;
      _logger.info("send scroll wheel delta: $_scrollWheelDelta");
      String payload = json.encode({
        'delta': _scrollWheelDelta.toInt()
      });
      bool result = _publisher.sendData('WinMobControl/ScrollMouseWheel', payload);
      if(result == false) {
        _logger.info("send scroll wheel delta failed");
      }

      _scrollWheelDelta = 0.0;
    }

    if(_zoomDeltaList.isNotEmpty) {
      // リストの最後の要素から最初の要素を引いた値を送信
      double delta = _zoomDeltaList.last - _zoomDeltaList.first;
      delta *= _zoomSensitivity;
      _logger.info("send zoom delta: $delta");
      String payload = json.encode({
        'delta': delta.toInt()
      });
      bool result = _publisher.sendData('WinMobControl/Zoom', payload);
      if(result == false) {
        _logger.info("send zoom delta failed");
      }

      _zoomDeltaList.clear();
    }

    if(_mouseMoveDelta.dx != 0.0 || _mouseMoveDelta.dy != 0.0) {
      _logger.info("send mouse move delta: $_mouseMoveDelta");
      _mouseMoveDelta *= _mouseSensitivity;
      String payload = json.encode({
        'delta_x': _mouseMoveDelta.dx.toInt(),
        'delta_y': _mouseMoveDelta.dy.toInt()
      });
      bool result = _publisher.sendData('WinMobControl/MoveMouseCursor', payload);
      if(result == false) {
        _logger.info("send mouse move delta failed");
      }

      _mouseMoveDelta = Offset(0.0, 0.0);
    }
  }
}