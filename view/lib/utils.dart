import 'package:flutter/material.dart';

// メッセージを表示するダイアログを表示する関数
Future<void> showAlertDialog(
  BuildContext context, {
  required String title,
  required String content,
  required String defaultActionText,
}) async {
  // ダイアログを表示する
  await showDialog<void>(
    context: context,
    builder: (context) => AlertDialog(
      title: Text(title),
      content: Text(content),
      actions: [
        TextButton(
          onPressed: () => Navigator.of(context).pop(),
          child: Text(defaultActionText),
        ),
      ],
    ),
  );
}