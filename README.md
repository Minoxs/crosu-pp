# CROSU-PP

This is a C-ish port of the great [rosu-pp](https://github.com/MaxOhn/rosu-pp) calculator thingy.
If possible, you should just use the native Rust version (it's awesome).

The only reason to use this, would be if you wish to call this code from some other language,
and the only option is going through C.

Obviously, the whole OOP stuff is gone through the roof almost immediately.

It would be possible to conserve _some_ ease of use from an OOP approach,
HOWEVER it's usually quite difficult and costly to call code from other languages, so I took a stupid simple
approach to "fix" that.

That is, expose all functionality through a simplified input->output API. Only passing over what is needed.
Thanks to this, it shouldn't be too hard to plop this library anywhere and use it.

P.s: Part of me wanted to name this "CRUST-PP" but that would obscure the heart of what I'm using, so it feels a bit cheap to do that.

# Building

Just run `cargo build --release` and you should have a shared library ready to use.

The headers can be found in the `bindings/` folder. I only tested it with the GNU GCC compiler, 
but it should be compatible with every other compiler.

To use this in your project will depend on language.
Feel free to contact me if you need help with the crimes needed to get this stuff up and running.

# Error Handling

Good luck

# Examples

To be added

# Thanks

To the maintainers of the [rosu-pp](https://github.com/MaxOhn/rosu-pp) calculator thingy,
I went through hell and back looking for something like this.

To ppy and the osu team :)
