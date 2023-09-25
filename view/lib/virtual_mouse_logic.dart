import 'package:flutter/material.dart';
import 'package:logging/logging.dart';
import "package:gesture_x_detector/gesture_x_detector.dart";

final _logger = Logger('VirtualMouseLogic');


class VirtualMouseLogic extends ChangeNotifier{
  bool _isConnected = false;
  // 各種処理に対応するキューを保有

  // websocketコネクション作成
  bool createConnection() {
    _isConnected = true;
    notifyListeners();
    _logger.info("create connection");
    // 接続成功したらtrueを返す
    return true;
  }

  // websocketコネクション削除
  bool deleteConnection() {
    _isConnected = false;
    notifyListeners();
    _logger.info("delete connection");
    // 接続解除成功したらtrueを返す
    return true;
  }

  // websocketコネクション確認
  bool isConnected() {
    return _isConnected;
  }

  // データ送信
  bool sendData() {
    // データ送信成功したらtrueを返す
    return true;
  }

  void onPushMouseLeftButton() {
    _logger.info("push left button");
  }

  void onPushMouseWheelButton() {
    _logger.info("push wheel button");
  }

  void onStartScrollMouseWheel(MoveEvent event) {
    _logger.info("start scroll wheel");
  }

  void onUpdateScrollMouseWheel(MoveEvent event) {
    _logger.info("update scroll wheel");
  }

  void onEndScrollMouseWheel(MoveEvent event) {
    _logger.info("end scroll wheel");
  }

  void onPushMouseRightButton() {
    _logger.info("push right button");
  }

  void onStartZoom(Offset event) {
    _logger.info("start zoom");
  }

  void onUpdateZoom(ScaleEvent event) {
    _logger.info("update zoom");
  }

  void onEndZoom() {
    _logger.info("end zoom");
  }

  void onStartMouseMove(MoveEvent event) {
    _logger.info("start move mouse");
  }

  void onUpdateMouseMove(MoveEvent event) {
    _logger.info("update move mouse");
  }

  void onEndMouseMove(MoveEvent event) {
    _logger.info("end move mouse");
  }

  void update() {
    // キューにデータが入っていれば適切な命令に書き換えて送信
  }

  
}