class Node:
    def __init__(self, value, next = None):
        self.value = value
        self.next = next

    def set_next(self, next):
        self.next = next


class LinkedList:
    def __init__(self):
        self.head = None
        
    def push(self, value):
        print("- Pushing value", value)
        node = Node(value=value)
        if self.head is None:
            self.head = node
            return
        
        last_node = self.head
        while last_node.next is not None:
            last_node = last_node.next
        last_node.set_next(node)
    
    def pop(self) -> Node:
        print("- Popping last value")
        # checks if was initialized
        if self.head is None:
            return None

        current = self.head

        if current.next is None:
            value = current.value
            self.head = None
            return value
        
        previous = None
        while current.next is not None:
            previous, current = current, current.next
        
        previous.next = current.next
        return current.value


    def __str__(self):
        return 'List: '+' => '.join([str(node.value) for node in self])

    def __iter__(self):
        current = self.head
        while current:
            yield current
            current = current.next

my_list = LinkedList()

for i in range(2, 10):
    my_list.push(i)
print(my_list)
print("Popped value: ", my_list.pop())
print("Popped value: ", my_list.pop())
print(my_list)
