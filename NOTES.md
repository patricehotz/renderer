# Notes

Two minutes at the end of each session. What confused me, what fixed it.
Raw material for blog posts later — capture it while it's still confusing.
Also i dont care abut spelling in those Notes at all. Will do that in the blog.

Format, loosely:

## YYYY-MM-DD — phase N

**Stuck on:**

**What it turned out to be:**

**Still don't understand:**

## 25.9.26 — phase 1

I just finished my first gradiant after like 8h of implementing.
Challenging at the beginning borrow etc
but i already did some rust and also c knowladge is helping me

i really like rust, holy! It sounds weird, but some aspects of js i really like but most i dont
i feel like rust combines the controll of c and the good aspects of js. 
idk when i googled if i could implement multiply for my own struct i was mind blown that there is actually a way and its not even hacky.

so i really reallly love rust. 

also i couldnt test the whole time if my gradiant logic and all other stuff works. and to my suprise it all worked in a first atempt.

## 26.9.26 — phase 2

Drawing a line is harder than you think. claude gave me a link to a guide that explores the different attempts of trying to draw a line. because its harder than you think.
I then tried to implement it on my own without the guide and after some confusion i finally cracked the code. i managed to draw a line.
the code ended up the pe the first step of the tutorial https://haqr.eu/tinyrenderer/bresenham/ and explores how it workes but it doesent look all to pretty. 
next illl work with the tutorial on how we can make it more accurate.
```rust
pub fn line(buffer: &mut FrameBuffer ,a: Coordinate, b: Coordinate, color: RGBA) {
    let vec = Vec2::calc_vector(a, b);
    let length = vec.calc_length() as u32;

    for t in 0..length {
        let t: f32 = t as f32 / length as f32;
        let p = a + vec * t;
        buffer.set_pixels(p, color)
    }
}
```

this is how the code currently looks. im exited on how the finished one will look like