import 'package:flutter/material.dart';
import 'package:nostr_sdk/nostr_sdk.dart';

Future<void> main() async {
  await Nip17.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    final pair = keys(
      hex: '6b911fd37cdf5c81d4c0adb1ab7fa822ed253ab0ad9aa18d77257c88b29b718e',
    );

    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
        body: Center(
          child: Column(
            children: [Text('secret: ${pair.$1}'), Text('pubkey: ${pair.$2}')],
          ),
        ),
      ),
    );
  }
}
