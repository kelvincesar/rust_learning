# Ownership and borrow checker summary

Here's a quick summary of some of the things you've mentioned that I think/hope might clear some stuff up for you.

Since you mentioned earlier in the thread you've done some C++, I think it's sometimes helpful to make comparisons relating back to that language. Rust's & and &mut references are most similar to C++'s regular pointer type, with two very important caveats: the borrowing system restricts when and where you can use each type, and a Rust reference must always be pointing towards a valid value of the underlying type. So essentially, we have the following correspondences:

let a = 5; // const int a = 5;

let a_ref = &a; // const int* const a_ptr = &a;

let mut a_ref_2 = &a; // const int* a_ptr_2 = &a;

let mut b = 6; //int b = 6;

let b_ref = &mut b; //int* const b_ptr = &b;

let mut b_ref_2 = &mut b; // int* b_ptr_2 = &b;

Now, we critically can only modify a value through a mutable reference. This is why if you look at the type signatures on methods/functions that modify what they're taking in or operating on, in Rust they are typically &mut self or &mut T for whatever type T we're looking at. This in many ways corresponds with non-const reference parameters on functions in C++.

Now, for the primary differences between Rust and C++ I mentioned earlier: let's start with the borrowing rules. At any one point in the flow of the program, we can either have multiple & references to a value OR a single mutable & mut reference to that value, but not both. Essentially, you can have multiple readers or a single modifier, and they're mutually exclusive. This doesn't mean that you cannot use both in the same block of code, but simply that they cannot be active at the same time; i.e.

let mut a = 5;

let a_ref = &a;

// a is locked and not modifiable (but can be borrowed more times with &) between these lines.

println!("{}", a_ref);

// a_ref dies here, so we can now use an &mut reference

let a_mut_ref = &mut a;

// underlying a is locked again and not modifiable except through a_mut_ref, and we cannot get more references to it.

*a_mut_ref += 1;

println!("{}", a);

Now, for the other aspect that's different from C++ I mentioned: Rust has no equivalent to null or nullptr so we cannot do anything that would cause a reference to be left in an invalid state. Namely, we cannot move a non-copy type out from behind a mutable reference, because this would leave the mutable reference as a pointer to an invalid state for the value it points to.

let mut a = 5;

let b = &mut a;

// Ok, because a here is i32 which is copy

let c = *b;

println!("{} {}", c, b);

let mut s = String::from("cakes");

let t = &mut s;

// Bad! We cannot move from behind t because is is String, which doesn't implement the copy trait

let v = *t;

println!("{} {}", v, t);

This difference between copy and non-copy types is important, because it lets us write really simple code when handling copy types (i.e. if you do any LeetCode problems with integer types, it's easier to fudge things because they're copy so you don't need to be as careful with what you do with mutable references), but it's also an easy way to get confused if you're not careful about keeping track of what references refer to copy values and which don't.

Now, a quick digression on as_ref and as_mut for the Option type. When we have something like head: Option<T> for some type T, it's important to note that head is referring to the outer wrapping Option, not what we are usually interested in, the T inside of head. Now, we can get to that T with .unwrap() supposing we know that head is not None, but doing so will move head in the case T is not copy (Option<T> is copy when T is copy and similarly not when T is not). This is often, as it was in this problem, not what we want to do because it means iterating down through a nested structure of some sort of Option<Box<Node>> will throw away the unwrapped values as we go!

For example, on a singly linked list, we can iterate past the tail via something like

// Pattern matching on Option<T> moves it!

while let Some(node) = head {

// Do stuff

head = node.next;
}

we're actually throwing out the nodes as well as their wrapping Option as we go! This is where as_ref and as_mut come in. They let us go from Option<T> (via implicitly borrowing to &Option<T> and &mut Option<T>respectively) toOption<&T>andOption<&mut T>` respectively. We can then destructure these objects via pattern matching move to get the underlying wrapped references without moving the original value, which preserves it!

// Doesn't consume the original values, just the references to them!

while let Some(node_ref) = head.as_ref() {

// Do stuff

head = &node.next; // Get an immutable reference for the next go around
}

I hope this was helpful. FWIW, I do agree with the other posters that Rust (due to this sort of nuance) isn't the greatest choice of language for LeetCoding when dealing with things like linked lists or trees; unless you're concerned with demonstrating proficiency in the language itself, there's probably better choices for (a) speed of writing up the solution and (b) not getting lost in the weeds of the language itself versus just solving the problem. That said, I do all my LeetCode in Rust, so it's certainly doable.
