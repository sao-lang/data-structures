package main

import "fmt"

func main() {
	// fmt.Println("Data Structures Library - Iterator Test")
	// fmt.Println("========================================")

	// fmt.Println("\n1. Singly Linked List:")
	// sll := NewSinglyLinkedList[int]()
	// sll.Append(1)
	// sll.Append(2)
	// sll.Append(3)
	// fmt.Printf("List: %v\n", sll.ToSlice())
	// fmt.Print("Iterating: ")
	// iter := sll.Iter()
	// for val, ok := iter(); ok; val, ok = iter() {
	// 	fmt.Printf("%d ", val)
	// }
	// fmt.Println()

	// fmt.Println("\n2. Circular Linked List:")
	// cll := NewCircularLinkedList[int]()
	// cll.Append(10)
	// cll.Append(20)
	// cll.Append(30)
	// fmt.Printf("List: %v\n", cll.ToSlice())
	// fmt.Print("Iterating: ")
	// iter = cll.Iter()
	// for val, ok := iter(); ok; val, ok = iter() {
	// 	fmt.Printf("%d ", val)
	// }
	// fmt.Println()

	// fmt.Println("\n3. Fixed Array:")
	// fa, _ := NewFixedArray[int](5)
	// fa.Push(100)
	// fa.Push(200)
	// fa.Push(300)
	// fmt.Printf("Array: %v\n", fa.ToArray())
	// fmt.Print("Iterating: ")
	// iter = fa.Iter()
	// for val, ok := iter(); ok; val, ok = iter() {
	// 	fmt.Printf("%d ", val)
	// }
	// fmt.Println()

	// fmt.Println("\n4. Stack:")
	// stack := NewStack[int]()
	// stack.Push(1)
	// stack.Push(2)
	// stack.Push(3)
	// fmt.Printf("Stack: %v\n", stack.ToSlice())
	// fmt.Print("Iterating: ")
	// iter = stack.Iter()
	// for val, ok := iter(); ok; val, ok = iter() {
	// 	fmt.Printf("%d ", val)
	// }
	// fmt.Println()

	// fmt.Println("\n5. Queue:")
	// queue := NewQueue[int]()
	// queue.Enqueue(10)
	// queue.Enqueue(20)
	// queue.Enqueue(30)
	// fmt.Printf("Queue: %v\n", queue.ToSlice())
	// fmt.Print("Iterating: ")
	// iter = queue.Iter()
	// for val, ok := iter(); ok; val, ok = iter() {
	// 	fmt.Printf("%d ", val)
	// }
	// fmt.Println()

	// fmt.Println("\nAll tests passed! ✓")
	num := NewInt(123).Add(NewInt(23))
	fmt.Println(num.value)
}
