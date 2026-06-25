import 'dart:async';

void main() {
  print('=== Dart Class 语法教程 ===\n');
  demoBasicClass();
  print('\n');
  demoInheritance();
  print('\n');
  demoMixins();
  print('\n');
  demoFuture();
}

// ========================================
// 1. 基本 Class 语法
// ========================================
void demoBasicClass() {
  print('--- 1. 基本 Class 语法 ---');
  
  // 创建 Person 实例
  final person = Person('张三', 25);
  person.sayHello();
  print('年龄: ${person.age}');
  
  // 使用命名构造函数
  final child = Person.child('小明');
  child.sayHello();
  
  // 使用 getter 和 setter
  person.age = 26;
  print('修改后的年龄: ${person.age}');
  print('是否成年: ${person.isAdult}');
}

class Person {
  // 字段
  final String name;
  int _age; // 私有字段（以下划线开头）
  
  // 主构造函数 - 简写形式
  Person(this.name, this._age);
  
  // 命名构造函数
  Person.child(this.name) : _age = 0;
  
  // 重定向构造函数
  Person.teenager(String name) : this(name, 15);
  
  // Getter
  int get age => _age;
  
  // Setter
  set age(int value) {
    if (value >= 0 && value <= 150) {
      _age = value;
    } else {
      print('无效的年龄');
    }
  }
  
  // 计算属性 Getter
  bool get isAdult => _age >= 18;
  
  // 方法
  void sayHello() {
    print('你好，我是 $name，今年 $_age 岁');
  }
  
  // 静态方法
  static void greet() {
    print('来自 Person 类的问候！');
  }
}

// ========================================
// 2. 继承 (Inheritance)
// ========================================
void demoInheritance() {
  print('--- 2. 继承 ---');
  
  final student = Student('李四', 20, '计算机科学');
  student.sayHello(); // 继承自 Person
  student.study(); // Student 自己的方法
}

class Student extends Person {
  final String major;
  
  // 子类构造函数，使用 super 调用父类构造函数
  Student(String name, int age, this.major) : super(name, age);
  
  @override
  void sayHello() {
    super.sayHello(); // 调用父类方法
    print('我的专业是 $major');
  }
  
  void study() {
    print('$name 正在学习 $major');
  }
}

// ========================================
// 3. 混合 (Mixins)
// ========================================
void demoMixins() {
  print('--- 3. 混合 (Mixins) ---');
  
  final bird = Bird();
  bird.fly();
  bird.walk();
  
  final fish = Fish();
  fish.swim();
}

// 定义一个 Mixin
mixin Flyable {
  void fly() {
    print('我可以飞！');
  }
}

mixin Swimmable {
  void swim() {
    print('我可以游泳！');
  }
}

mixin Walkable {
  void walk() {
    print('我可以走路！');
  }
}

// 使用 with 关键字应用 Mixin
class Bird with Walkable, Flyable {}

class Fish with Swimmable {}

// ========================================
// 4. Future (异步编程)
// ========================================
void demoFuture() async {
  print('--- 4. Future (异步编程) ---');
  
  print('开始异步操作...');
  
  // 方式 1: 使用 .then() 和 .catchError()
  print('\n方式 1: 使用 .then()');
  fetchData().then((data) {
    print('收到数据: $data');
  }).catchError((error) {
    print('发生错误: $error');
  });
  
  // 方式 2: 使用 async/await (推荐)
  print('\n方式 2: 使用 async/await');
  try {
    final data = await fetchData();
    print('收到数据: $data');
  } catch (error) {
    print('发生错误: $error');
  }
  
  // 并行执行多个 Future
  print('\n并行执行多个 Future:');
  final results = await Future.wait([
    fetchDataWithDelay('数据 1', 1),
    fetchDataWithDelay('数据 2', 2),
    fetchDataWithDelay('数据 3', 1),
  ]);
  print('所有结果: $results');
  
  // Future 的其他方法
  print('\nFuture 的其他方法:');
  final firstResult = await Future.any([
    fetchDataWithDelay('慢速数据', 3),
    fetchDataWithDelay('快速数据', 1),
  ]);
  print('第一个完成的结果: $firstResult');
}

// 返回 Future 的函数
Future<String> fetchData() {
  return Future.delayed(Duration(seconds: 2), () {
    return 'Hello, Dart Future!';
  });
}

Future<String> fetchDataWithDelay(String data, int seconds) {
  return Future.delayed(Duration(seconds: seconds), () {
    print('完成: $data (耗时 $seconds 秒)');
    return data;
  });
}

// 可能抛出错误的 Future
Future<String> fetchDataWithError() {
  return Future.delayed(Duration(seconds: 1), () {
    throw Exception('网络请求失败！');
  });
}

// ========================================
// 总结
// ========================================
/*
Dart Class 语法要点：
1. 使用 class 关键字定义类
2. 构造函数：主构造函数、命名构造函数、重定向构造函数
3. 私有成员：以下划线 _ 开头
4. Getter 和 Setter：使用 get 和 set 关键字
5. 继承：使用 extends，super 调用父类
6. Mixins：使用 with，实现代码复用

Future 要点：
1. 表示异步操作的结果
2. 三种状态：未完成、完成、出错
3. 使用 .then()/.catchError() 或 async/await 处理
4. Future.wait() 并行执行多个 Future
5. Future.any() 获取第一个完成的结果
*/
