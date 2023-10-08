import 'dart:async';
import 'dart:io';

import 'package:flutter/services.dart';
import 'package:mqtt5_client/mqtt5_client.dart';
import 'package:mqtt5_client/mqtt5_server_client.dart';
import 'package:uuid/uuid.dart';
import 'package:logging/logging.dart';

final _logger = Logger('MQTTPublisher');

// MQTTのパブリッシャーをクラスとして実装
class MQTTPublisher {
  // 接続状態
  bool _isConnected = false;
  // IPアドレス
  String _ipAddress = '';
  // ポート番号
  int _port = 0;
  // MQTTクライアント
  MqttServerClient? _client;

  // constructor
  MQTTPublisher() {
    // 初期化処理
  }

  // mqtt接続
  Future<bool> createConnection(String ipAddress, int port) async {
    _ipAddress = ipAddress;
    _port = port;
    // クライアントの生成
    _client = MqttServerClient.withPort(_ipAddress, const Uuid().v1(), _port);
    _client?.logging(on: false);
    _client?.keepAlivePeriod = 20;

    // SSL設定
    _client?.secure = true;
    SecurityContext securityContext = SecurityContext.defaultContext;
    // CA証明書をassetsから読み込み（assets/ca.crt）
    final caFile = await rootBundle.load('assets/ca.crt');
    // securityContextへ登録
    securityContext.setTrustedCertificatesBytes(caFile.buffer.asUint8List());
    // 秘密鍵をassetから読みこみ（assets/mob-client.key）
    final clientKeyFile = await rootBundle.load('assets/mob-client.key');
    // securityContextへ登録
    securityContext.usePrivateKeyBytes(clientKeyFile.buffer.asUint8List());
    // クライアント証明書をassetsから読み込み（assets/mob-client.pem）
    final clientCertFile = await rootBundle.load('assets/mob-client.crt');
    securityContext.useCertificateChainBytes(clientCertFile.buffer.asUint8List());
    _client?.securityContext = securityContext;

    // これを有効化すると問答無用で接続できるようになる
    // _client?.onBadCertificate = (dynamic certificate) {
    //   _logger.info('Bad certificate');
    //   return true;
    // };

    // 接続時設定
    final connMess = MqttConnectMessage()
      .withClientIdentifier('MQTT5DartClient')
      .startClean();
    _client?.connectionMessage = connMess;

    // 接続処理
    try {
      await _client?.connect();
    } on Exception catch (e) {
      _logger.severe('MQTT client exception - $e');
      _client?.disconnect();
    }

    // 接続確認
    if (_client?.connectionStatus!.state == MqttConnectionState.connected) {
      _logger.info('MQTT client connected');
      _isConnected = true;

      return true;
    } else {
      _logger.info('ERROR MQTT client connection failed - disconnecting, state is ${_client?.connectionStatus!.state}');
      _client?.disconnect();
      _isConnected = false;

      return false;
    }
  }

  // mqtt切断
  bool deleteConnection() {
    _client?.disconnect();
    _isConnected = false;

    // 接続確認
    if (_client?.connectionStatus!.state == MqttConnectionState.connected) {
      _isConnected = true;
      _logger.info('ERROR MQTT client connection failed - disconnecting, state is ${_client?.connectionStatus!.state}');
      return false;
    }

    return true;
  }

  // 接続状態
  bool isConnected() {
    return _isConnected;
  }

  // データ送信
  bool sendData(String topic, String data) {
    if(!_isConnected) {
      return false;
    }

    final builder = MqttPayloadBuilder();
    builder.addString(data);
    _client?.publishMessage(topic, MqttQos.atLeastOnce, builder.payload!);

    return true;
  }
}