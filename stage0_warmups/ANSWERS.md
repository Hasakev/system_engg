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

> it fails not because &string is a borrow pointing to the original address, but because it can't keep the bytes alive, when main is dead, i.e. ownership is returned, then main dies, so \[h e l l o\] dies.
For print_len function to take ownership, it would need to take s: String. However what that means is that main would no longer be able to use s again, and the print function would fail since main no longer has ownership.
3. third

### b2 push while borrowed

### b3 dangling

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
