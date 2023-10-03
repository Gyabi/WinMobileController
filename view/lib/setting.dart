// 設定画面ダイアログを定義

import 'package:flutter/material.dart';
import 'package:logging/logging.dart';

final _logger = Logger('SettingDialog');

// 設定画面ダイアログ
class SettingDialog extends StatefulWidget {
  const SettingDialog({Key? key}) : super(key: key);

  @override
  _SettingDialogState createState() => _SettingDialogState();
}

class _SettingDialogState extends State<SettingDialog> {
  // IPアドレス
  String _ipAddress = '';
  // ポート番号
  int _port = 0;
  // ホイール感度スライダの値
  double _wheelSensitivity = 1.0;
  // ズーム感度スライダの値
  double _zoomSensitivity = 1.0;
  // マウス感度スライダの値
  double _mouseSensitivity = 1.0;

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text('Setting'),
      content: SingleChildScrollView(
        child: Column(
          children: [
            // サーバIPアドレス入力
            Container(
              margin: const EdgeInsets.only(bottom: 10.0),
              child: TextField(
                decoration: InputDecoration(
                  filled: true,
                  fillColor: Colors.grey.shade200,
                  border: OutlineInputBorder(
                    borderRadius: BorderRadius.circular(10),
                    borderSide: BorderSide.none,
                  ),
                  labelText: "Server IP Address"
                ),
                onChanged: (value) => {
                  _logger.info('Server IP Address: $value'),
                  setState(() {
                    _ipAddress = value;
                  })
                },
                onEditingComplete: () => {
                  _logger.info('Server IP Address: $_ipAddress'),
                  // TODO：保存処理
                },
              ),
            ),

            // サーバポート入力
            Container(
              margin: const EdgeInsets.only(bottom: 10.0),
              child: TextField(
                decoration: InputDecoration(
                  filled: true,
                  fillColor: Colors.grey.shade200,
                  border: OutlineInputBorder(
                    borderRadius: BorderRadius.circular(10),
                    borderSide: BorderSide.none,
                  ),
                  labelText: "Server Port"
                ),
                onChanged: (value) => {
                  _logger.info('Server Port: $value'),
                  setState(() {
                    _port = int.parse(value);
                  })
                },
                onEditingComplete: () => {
                  _logger.info('Server Port: $_port'),
                  // TODO：保存処理
                },
              ),
            ),

            // ホイール感度スライダ
            Container(
              margin: const EdgeInsets.only(bottom: 10.0),
              child: Column(
                children: [
                  Slider(
                    value: _wheelSensitivity,
                    min: 0.0,
                    max: 3.0,
                    divisions: 10,
                    label: 'Wheel Sensitivity',
                    onChanged: (double value) {
                      _logger.info('Wheel Sensitivity: $value');
                      setState(() {
                        _wheelSensitivity = value;
                      });
                    },
                    onChangeEnd: (double value) {
                      _logger.info('Wheel Sensitivity: $value');
                      setState(() {
                        _wheelSensitivity= value;
                      });
                      // TODO：保存処理
                    },
                  ),
                  const Text('Wheel Sensitivity'),
                ],
              ) 
            ),

            // ズーム感度スライダ
            Container(
              margin: const EdgeInsets.only(bottom: 10.0),
              child: Column(
                children: [
                  Slider(
                    value: _zoomSensitivity,
                    min: 0.0,
                    max: 3.0,
                    divisions: 10,
                    label: 'Zoom Sensitivity',
                    onChanged: (double value) {
                      _logger.info('Zoom Sensitivity: $value');
                      setState(() {
                        _zoomSensitivity = value;
                      });
                    },
                    onChangeEnd: (double value) {
                      _logger.info('Zoom Sensitivity: $value');
                      setState(() {
                        _zoomSensitivity = value;
                      });
                      // TODO：保存処理
                    },

                  ),
                  const Text('Zoom Sensitivity'),
                ],
              ) 
            ),

            // マウス感度スライダ
            Container(
              margin: const EdgeInsets.only(bottom: 10.0),
              child: Column(
                children: [
                  Slider(
                    value: _mouseSensitivity,
                    min: 0.0,
                    max: 3.0,
                    divisions: 10,
                    label: 'Mouse Sensitivity',
                    onChanged: (double value) {
                      _logger.info('Mouse Sensitivity: $value');
                      setState(() {
                        _mouseSensitivity = value;
                      });
                    },
                    onChangeEnd: (double value) {
                      _logger.info('Mouse Sensitivity: $value');
                      setState(() {
                        _mouseSensitivity = value;
                      });
                      // TODO：保存処理
                    },
                  ),
                  const Text('Mouse Sensitivity'),
                ],
              ) 
            ),
          ],
        ),
      ),
      actions: [
        TextButton(
          onPressed: () {
            // ダイアログを閉じる
            Navigator.of(context).pop();
          },
          child: const Text('Close'),
        ),
      ],
    );
  }
}