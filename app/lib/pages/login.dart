import 'package:flutter/material.dart';

class LoginPage extends StatelessWidget {
  LoginPage({super.key});
  

  final Widget loginForm = ListView(
    padding: const EdgeInsets.all(16.0),
    children: [
      
      const SizedBox(height: 16.0),

      Text('E-mailadres:'),
      TextFormField(
        validator: (value) {
          if (value == null || value.isEmpty) {
            return 'Please enter some text';
          }
          return null;
        },
        decoration: const InputDecoration(
          border: OutlineInputBorder(),
          hintText: 'example@email.com',
        ),
      ),
      const SizedBox(height: 16.0),

      Text('Password:'),
      TextField(
        decoration: const InputDecoration(
          border: OutlineInputBorder(),
          hintText: 'yourpassword',
        ),
        obscureText: true,
      ),
      const SizedBox(height: 16.0),

      ElevatedButton(
        onPressed: () {
          // Handle login logic
        },
        child: const Text('Login'),
      ),
    ],
  );

  final Widget registerForm = ListView(
    padding: const EdgeInsets.all(16.0),
    children: [
      const SizedBox(height: 16.0),
      Text('Register code:'),
      TextField(
        decoration: const InputDecoration(
          border: OutlineInputBorder(),
          hintText: 'XXXXX',
        ),
      ),
      const SizedBox(height: 16.0),
      Text('E-mailadres:'),
      TextFormField(
        validator: (value) {
          if (value == null || value.isEmpty) {
            return 'Please enter some text';
          }
          return null;
        },
        decoration: const InputDecoration(
          border: OutlineInputBorder(),
          hintText: 'example@email.com',
        ),
      ),
      const SizedBox(height: 16.0),

      Text('Password:'),
      TextField(
        decoration: const InputDecoration(
          border: OutlineInputBorder(),
          hintText: 'yourpassword',
        ),
        obscureText: true,
      ),
    
      const SizedBox(height: 16.0),
      ElevatedButton(
        onPressed: () {
          // Handle login logic
        },
        child: const Text('Login'),
      ),
    ],
  );


  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: Container(
          padding: const EdgeInsets.all(20.0),
          child: Column(
            spacing: 20,
            children: <Widget>[
              ClipRRect(
                borderRadius: BorderRadius.circular(12.0),
                child: Image(image: AssetImage('assets/banner.png')),
              ),

              Text('Login to IMS'),
              
              Expanded(
                child: DefaultTabController(
                  length: 2,
                    child: Column(
                      children: [
                        const TabBar(
                            tabs: [
                              Tab(text: "Login"),
                              Tab(text: "Register"),
                            ],
                          ),
                        Expanded(
                          child: TabBarView(
                            children: [
                              loginForm,
                              registerForm
                            ],
                          ),
                        ),
                      ],
                    ),
                  ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

