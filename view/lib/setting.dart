// 設定画面ダイアログを定義
import 'package:flutter/material.dart';
import 'package:logging/logging.dart';
import "virtual_mouse_logic.dart";
import 'package:provider/provider.dart';
import 'package:shared_preferences/shared_preferences.dart';

final _logger = Logger('SettingDialog');

// 設定画面ダイアログ
class SettingDialog extends StatefulWidget {
  const SettingDialog({Key? key}) : super(key: key);

  @override
  _SettingDialogState createState() => _SettingDialogState();
}

class _SettingDialogState extends State<SettingDialog> {
  // 各種設定値
  // IPアドレス
  TextEditingController _ipAddress = TextEditingController();
  // ポート番号
  TextEditingController _port = TextEditingController();
  // ホイール感度
  double _wheelSensitivity = 1.0;
  // ズーム感度
  double _zoomSensitivity = 1.0;
  // マウス感度
  double _mouseSensitivity = 1.0;

  @override
  void initState() {
    super.initState();
    // 初期化処理
    _init();
  }

  // 初期化処理
  Future<void> _init() async {
    SharedPreferences pref = await SharedPreferences.getInstance();
    setState(() {
      // IPアドレスを取得
      _ipAddress.text = pref.getString('ipAddress') ?? '';
      // ポート番号を取得
      _port.text = pref.getInt('port').toString(); 
      // ホイール感度を取得
      _wheelSensitivity = pref.getDouble('wheelSensitivity') ?? 1.0;
      // ズーム感度を取得
      _zoomSensitivity = pref.getDouble('zoomSensitivity') ?? 1.0;
      // マウス感度を取得
      _mouseSensitivity = pref.getDouble('mouseSensitivity') ?? 1.0;
    });
  }

  Future<void> _save() async {
    SharedPreferences pref = await SharedPreferences.getInstance();
    // IPアドレスを保存
    pref.setString('ipAddress', _ipAddress.text);
    // ポート番号を保存
    pref.setInt('port', int.parse(_port.text));
    // ホイール感度を保存
    pref.setDouble('wheelSensitivity', _wheelSensitivity);
    // ズーム感度を保存
    pref.setDouble('zoomSensitivity', _zoomSensitivity);
    // マウス感度を保存
    pref.setDouble('mouseSensitivity', _mouseSensitivity);
  } 

  @override
  Widget build(BuildContext context) {
    // Providerよりロジッククラスのインスタンスを取得
    final logic = Provider.of<VirtualMouseLogic>(context, listen: true);

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
                onChanged: (value) {
                  _logger.info('Server IP Address: $value');
                  setState(() {
                    _ipAddress.text = value;
                  });
                },
                controller: _ipAddress,
              ),
            ),

            // サーバポート入力
            Container(
              margin: const EdgeInsets.only(bottom: 10.0),
              child: TextField(
                // 入力は数字だけ許可
                keyboardType: TextInputType.number,
                decoration: InputDecoration(
                  filled: true,
                  fillColor: Colors.grey.shade200,
                  border: OutlineInputBorder(
                    borderRadius: BorderRadius.circular(10),
                    borderSide: BorderSide.none,
                  ),
                  labelText: "Server Port"
                ),
                onChanged: (value) {
                  _logger.info('Server Port: $value');
                  setState(() {
                    _port.text = value;
                  });
                },
                controller: _port,
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
          onPressed: () async {
            // 設定値を保存
            await _save();
            // ロジッククラスに設定値を反映
            await logic.readParams();

            if (!mounted) {
              return;
            }
            // ダイアログを閉じる
            Navigator.of(context).pop();
          },
          child: const Text('Close'),
        ),
      ],
    );
  }
}