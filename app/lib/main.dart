
import 'package:app/pages/login.dart';
import 'package:flutter/material.dart';



void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: LoginPage()
      );
  }
}

// class MyApp extends StatelessWidget {
//   const MyApp({super.key});
//   final int test = 0;

//   @override
//   Widget build(BuildContext context) {
//     return MaterialApp(
//       title: 'Flutter Demo',
//       home: Scaffold(
//         bottomNavigationBar: BottomNavigationBar(
//         items: const <BottomNavigationBarItem>[
//           BottomNavigationBarItem(icon: Icon(Icons.home), label: 'Home'),
//           BottomNavigationBarItem(icon: Icon(Icons.business), label: 'Business'),
//           BottomNavigationBarItem(icon: Icon(Icons.school), label: 'Schoolll'),
//         ],
//       ),
//       ),
//     );
//   }
// }










