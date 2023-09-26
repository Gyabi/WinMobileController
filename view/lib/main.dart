import 'package:flutter/material.dart';
import "./virtual_mouse.dart";
import "package:logging/logging.dart";

void main() {
  // ログ設定
  Logger.root.level = Level.ALL;
  Logger.root.onRecord.listen((record) {
    print('${record.level.name}: ${record.time}: ${record.loggerName}: ${record.message}');
  });

  // エントリポイント
  runApp(const MyApp());
}

// ルートwidgetの定義
class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Win Mobile Controller',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: const VirtualMousePage(title: 'Win Mobile Controller'),
    );
  }
}


