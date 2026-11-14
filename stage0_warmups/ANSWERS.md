# Stage 0 answers

Write your prediction before you code or compile, then add what you learned
after. Keep both: the gap between them is the useful part.

Format for each question:

```
Q: (copy the question)
Predicted:
Actual / learned:
```

## borrowck

### b1 use after move
prediction: it will get mad, when it reaches line 13, as main no longer has access to s
> correction: error happens at compile time, line 13 isn't the correct terminology because code is not compield!

actual error: borrow of moved value 's', move occurs because s has type 'String', which doesn't have "Copy", then the value is moved during print_len, then during print which has inherent borrowing, tries to borrow but then fails. the note tells us to borrow instead of using the true value, and suggests using clone (which we don't use)

fix: make print_len borrow s instead.
so "s" owns the string, but when i fed s into print_len previously, print_len took ownership of s, then when the function terminated, s gets released, hence main cannot access it anymore.

To counter this, I instead changed print_len to take in the address of s: &s, hence s still owns s, and lends it's access to the print_len function, hence we can later print it!

why? If print_len had to keep the string after returning, would &String work, and what would you pass instead?

if print_len had to keep the string after returning we could not use &String, since &string is a borrow pointing to the original address.

> it fails not because &string is a borrow pointing to the original address, but because it can't keep the bytes alive as s is simply borrowed, s then goes out of scope (i.e. reaches the end of its block and is dropped), so \[h e l l o\] dies.
For print_len function to take ownership, it would need to take s: String. However what that means is that main would no longer be able to use s again, and the print function would fail since main no longer has ownership.

### b2 push while borrowed

prediction: v gets assigned initially, then first borrows v:
v = *n = [ 1 2 3 ]
first borrows v:
first -> n address 
n address relocates to expand array size.
first still point to old address
print gets cooked

v tries to push 4, but because *first* is still borrowing v, it fails

error comes up on the line 12.

reality: a mutable borrow occurs here error happens. If first wasn't around this would work, because v is mutable, however because first uses &v[0], v becomes immutable as we have borrowed v, hence it gets cooked

fix: do the mutable change before the immutable assingmenta i.e. print before pushing, another method is simply, instead of point to the address of first, we can just get first to become v\[0\], a copy. first then lives as an int on the stack, popped off the stack when the scope ends i.e. main hits }, v is also freed, when the system ends.
 
C is cooked because what happens is that the nothing stops the expansion, the pointer to first dangles and so if you call for it you potentially get garbage from another item stored in that address.

the heap expands, because 1. something else might fall after this heap i.e. address = 0x1000, 0x1004, 0x1008, 0x100C, then address 11 something else is there. so we need to find a new block somewhere that its 4. once it's copied over you nede to free the old block and point the new pointer to it. 

So if someone else comes along and tries to access first, they still point to the old address and rubbish


### b3 dangling
prediction: text points to some string bytes "hello\nworld". not sure what unwrap_or does. but essentially it will throw an error because upper is within the context of the first_line_upper function, and so when the function reaches }, it disppears, note we haven't returned anything, and so the function will fail.

result: it seems like the upper.lines.next().unwrap_or("") function returns something to the main caller. (return is not needed in the rust language). However, upper is owned by the function and so if first_line_upper finishes at }, the memory of upper gets cleared, and the caller, main, can no longer access the borrowed value. To fix this, we need to make main take ownership of upper. 

1. What type is the value upper.lines().next().unwrap_or(""), and does it own its bytes?
type is &str, it does no own the bytes, upper does
2. What does putting ; after it do?
putting a ; after it, turns a &str type to a () type because it converts it to a statement.
3. What method turns that value into something that owns its bytes?
we need to assign the value of the first line upper to a new owned object that is stored in the heap.

Your fix allocates a second heap block and copies 5 bytes. The original broken version tried to avoid that copy. Is there any signature where returning a &str would be fine? Hint: what if the slice pointed into text instead of upper? Who owns text's bytes, and how long do they live?

text is owned by main, so if i view into text, that's fine. However, if i want to manipulate text, that's another story, when i run the to_uppercase function, it creates a new variableon the heap that get removed when the function dies, so the results need to be in an owned string to pass it back, and so I can't do that operation, i need to make a new object to do this, hence we had upper allocated in the helper function. Therefore, we cannot get first_line_upper by using &text since we are just viewing it.

### b4 missing lifetime

### b5 mutate while iterating

### b6 move out of borrow

## e01 ownership

## e02 slices

## e03 bits

## e04 enums

## e05 errors

## e06 generics

## e07 lifetimes

## e08 boxes

## e09 unsafe

## progress
