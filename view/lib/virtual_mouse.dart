import 'package:flutter/material.dart';
import 'package:logging/logging.dart';

final _logger = Logger('VirtualMousePage');

// 仮想マウスを表示するページ
class VirtualMousePage extends StatelessWidget {
  
  const VirtualMousePage({Key? key, required this.title}) : super(key: key);

  final String title;

  @override
  Widget build(BuildContext context) {
    // 画面の定義
    return Scaffold(
      // アプリバー
      appBar: AppBar(
        title: Text(title),
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
      ),
      // ボディ
      body: const Center(
        child: ConnectControlWidget(),
      )
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
  bool _isConnected = false;

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Container(
          margin: const EdgeInsets.all(20.0),
          child: Row(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Container(
                margin: const EdgeInsets.all(10.0),
                child: ElevatedButton(
                  onPressed: () {
                    // TODO:ここに接続処理を実装する
                    setState(() {
                      _isConnected = !_isConnected;
                    });
                  },
                  child: Text(_isConnected ? 'Disconnect' : 'Connect'),
                ),
              ),
              Container(
                margin: const EdgeInsets.all(10.0),
                child: Text(
                  _isConnected ? 'CONNECTING' : 'DISCONNECTED',
                  style: TextStyle(
                    fontSize: 20,
                    fontWeight: FontWeight.bold,
                    color: _isConnected ? Colors.green : Colors.red,
                  ),
                ),
              ),
            ],
          ),
        ),
        const Divider(),
        // 接続状態のみUI表示
        Visibility(
          visible: _isConnected,
          child: Container(
            margin: const EdgeInsets.all(20.0),
            child: const MouseControlWidget(),
          ),
        )
      ],
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

    return Column(
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
                  // TODO:ここに左ボタン押下処理を記述する
                  _logger.info("push left button");
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
              GestureDetector(
                onTap: () => {
                  _logger.info("push wheel button")
                },
                onVerticalDragStart: (details) => {
                  _logger.info("start scroll")
                },
                onVerticalDragUpdate: (details) => {
                  _logger.info("scrolling")
                },
                onVerticalDragEnd: (details) => {
                  _logger.info("end scroll")
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
                  // TODO:ここに右ボタン押下処理を記述する
                  _logger.info("push right button");
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
        GestureDetector(
          onTap: () => {
            _logger.info("push mouse")
          },
          onScaleStart: (details) => {
            _logger.info("start zoom")
          },
          onScaleUpdate: (details) => {
            _logger.info("zooming")
          },
          onScaleEnd: (details) => {
            _logger.info("end zoom")
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
    ); 
  }
}
