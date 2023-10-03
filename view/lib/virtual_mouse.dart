import 'package:flutter/material.dart';
import "package:gesture_x_detector/gesture_x_detector.dart";
import "virtual_mouse_logic.dart";
import 'package:provider/provider.dart';
import "setting.dart";

// 仮想マウスを表示するページ
class VirtualMousePage extends StatelessWidget {
  // constructor
  const VirtualMousePage({Key? key, required this.title}) : super(key: key);

  final String title; //constructor引数はfinalで宣言

  @override
  Widget build(BuildContext context) {
    // 画面の定義
    return ChangeNotifierProvider<VirtualMouseLogic>( //ChangeNotifierProviderがProviderパターンを強制実装
      create: (_) => VirtualMouseLogic(), //伝搬したいインスタンスをここで指定
      child: Scaffold( //childでは自由にPoviderからインスタンスを取得できるようになる
        appBar: AppBar(
          title: Text(
            title,
            style: TextStyle(
                color: Theme.of(context).colorScheme.primary,
                fontWeight: FontWeight.bold,
                fontSize: 20.0,
              ),
            ),
          backgroundColor: Theme.of(context).colorScheme.inversePrimary,
          actions: const [
            SettingButton(),
          ],
        ),
        // ボディ
        body: const SingleChildScrollView(
          child: Column(
            children: [
              // 接続制御UI
              ConnectControlWidget(),
              Divider(), //区切り線
              // 疑似マウスUI
              MouseControlWidget()
            ],
          ),
        )
      )
    );
  }
}

// 設定ダイアログ表示ボタン
class SettingButton extends StatelessWidget {
  const SettingButton({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return IconButton(
      onPressed: () {
        // 設定ダイアログを表示
        showSettingDialog(context);
      },
      icon: const Icon(Icons.settings),
      color: Theme.of(context).colorScheme.primary,
    );
  }

  void showSettingDialog(BuildContext context) {
    final logic = Provider.of<VirtualMouseLogic>(context, listen: false);

    showDialog(
      context: context,
      builder: (_) {
        // ダイアログにProviderを適応させる場合はChangeNotifierProvider.valueを使う
        return ChangeNotifierProvider.value(value: logic, child: const SettingDialog());
      }
    );
  }
}


// 接続制御UI（ボタンおよび状態表示テキスト）
class ConnectControlWidget extends StatefulWidget {
  const ConnectControlWidget({super.key});

  @override
  State<ConnectControlWidget> createState() => _ConnectControlWidgetState();
}

class _ConnectControlWidgetState extends State<ConnectControlWidget> {
  @override
  Widget build(BuildContext context) {
    // Providerよりロジッククラスのインスタンスを取得
    final logic = Provider.of<VirtualMouseLogic>(context, listen: true);

    return Container(
          margin: const EdgeInsets.all(20.0),
          child: Row(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Container(
                margin: const EdgeInsets.all(10.0),
                child: ElevatedButton(
                  onPressed: () {
                    if (logic.isConnected()) {
                      logic.deleteConnection();
                    } else {
                      logic.createConnection();
                    }
                  },
                  child: Text(logic.isConnected() ? 'Disconnect' : 'Connect'),
                ),
              ),
              Container(
                margin: const EdgeInsets.all(10.0),
                child: Text(
                  logic.isConnected() ? 'CONNECTING' : 'DISCONNECTED',
                  style: TextStyle(
                    fontSize: 20,
                    fontWeight: FontWeight.bold,
                    color: logic.isConnected() ? Colors.green : Colors.red,
                  ),
                )
              )
            ]
          )
    );
  }
}


// 疑似マウスUI（左クリックボタン、右クリックボタン、ホイールボタン、マウス移動用ジェスチャーディテクタ）
// 受け付けるアクションは、左クリック、右クリック、ホイールクリック、マウス移動、二本指でホイール回転、ピンチインアウトでズーム
class MouseControlWidget extends StatefulWidget {
  const MouseControlWidget({super.key});

  @override
  State<MouseControlWidget> createState() => _MouseControlWidgetState();
}

class _MouseControlWidgetState extends State<MouseControlWidget> {

  @override
  Widget build(BuildContext context) {
    // 画面サイズの80%を疑似マウスエリアとする
    double areaWidth = MediaQuery.of(context).size.width * 0.8;
    // ロジッククラスのインスタンスを取得
    final logic = Provider.of<VirtualMouseLogic>(context, listen: true);

    return Visibility(
      visible: logic.isConnected(),
      child: Container(
        margin: const EdgeInsets.all(20.0),
        child: Column(
          children: [
            Container(
              height: areaWidth * 0.4,
              width: areaWidth,
              child: Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  // 左クリックボタン
                  ElevatedButton(
                    onPressed: () {
                      logic.onPushMouseLeftButton();
                    },
                    style: ElevatedButton.styleFrom(
                      shape: RoundedRectangleBorder(
                        borderRadius: BorderRadius.circular(10),
                      )
                    ),
                    child: const Text('LEFT'),
                  ),

                  // スクロールホイール
                  // タップ→ホイールクリック
                  // 上下スワイプ→スクロール
                  XGestureDetector(
                    onTap: (TapEvent event) => {
                      logic.onPushMouseWheelButton()
                    },
                    onMoveStart: (MoveEvent event) => {
                      logic.onStartScrollMouseWheel(event)
                    },
                    onMoveUpdate: (MoveEvent event) => {
                      logic.onUpdateScrollMouseWheel(event)
                    },
                    onMoveEnd: (MoveEvent event) => {
                      logic.onEndScrollMouseWheel(event)
                    },

                    child: Container(
                      width: areaWidth * 0.2,
                      margin: const EdgeInsets.all(20.0),
                      padding: const EdgeInsets.all(20.0),
                      decoration: BoxDecoration(
                        color: Colors.grey,
                        borderRadius: BorderRadius.circular(10),
                      ),
                    ),
                  ),

                  // 右クリックボタン
                  ElevatedButton(
                    onPressed: () {
                      logic.onPushMouseRightButton();
                    },
                    style: ElevatedButton.styleFrom(
                      shape: RoundedRectangleBorder(
                        borderRadius: BorderRadius.circular(10),
                      )
                    ),
                    child: const Text('RIGHT'),
                  ),
                ],
              ),
            ),

            // クリック→右クリック
            // ピンチインアウト→拡大縮小
            XGestureDetector(
              onTap: (TapEvent event) => {
                logic.onPushMouseLeftButton()
              },
              onScaleStart: (Offset event) => {
                logic.onStartZoom(event)
              },
              onScaleUpdate: (ScaleEvent event) => {
                logic.onUpdateZoom(event)
              },
              onScaleEnd: () => {
                logic.onEndZoom()
              },
              onMoveStart: (MoveEvent event) => {
                logic.onStartMouseMove(event)
              },
              onMoveUpdate: (MoveEvent event) => {
                logic.onUpdateMouseMove(event)
              },
              onMoveEnd: (MoveEvent event) => {
                logic.onEndMouseMove(event)
              },

              child: Container(
                height: areaWidth,
                width: areaWidth,
                margin: const EdgeInsets.all(20.0),
                padding: const EdgeInsets.all(20.0),
                decoration: BoxDecoration(
                  color: Colors.grey,
                  borderRadius: BorderRadius.circular(10),
                ),
              ),
            )
          ],
        )
      )
    );
    
  }
}
