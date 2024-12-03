// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

(String, String) keys({required String hex}) =>
    Nip17.instance.api.crateApiNostrKeys(hex: hex);

Future<String> encodeNip17(
        {required String senderSecret,
        required String receiverPublic,
        required String message}) =>
    Nip17.instance.api.crateApiNostrEncodeNip17(
        senderSecret: senderSecret,
        receiverPublic: receiverPublic,
        message: message);

Future<(String, String)?> decodeNip17(
        {required String receiverSecretKey, required String eventJson}) =>
    Nip17.instance.api.crateApiNostrDecodeNip17(
        receiverSecretKey: receiverSecretKey, eventJson: eventJson);

Future<String> sendNip17(
        {required String senderSecretKey,
        required String receiverPublicKey,
        required String message,
        required String relay}) =>
    Nip17.instance.api.crateApiNostrSendNip17(
        senderSecretKey: senderSecretKey,
        receiverPublicKey: receiverPublicKey,
        message: message,
        relay: relay);

Uint8List sign(
        {required List<int> signerSecretKey, required List<int> message}) =>
    Nip17.instance.api
        .crateApiNostrSign(signerSecretKey: signerSecretKey, message: message);

bool verify(
        {required List<int> signerPublicKey,
        required List<int> message,
        required List<int> signature}) =>
    Nip17.instance.api.crateApiNostrVerify(
        signerPublicKey: signerPublicKey,
        message: message,
        signature: signature);