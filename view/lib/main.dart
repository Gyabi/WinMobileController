import 'package:flutter/material.dart';
import "./virtual_mouse.dart";

void main() {
  runApp(const MyApp());
}

// ルートwidgetの定義
class MyApp extends StatelessWidget {
  const MyApp({super.key});

  // This widget is the root of your application.
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


