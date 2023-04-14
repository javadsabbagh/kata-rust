# Deref `Deref`shì `deref`cāozuò fú `*`de trait, bǐrú `*v`. Yībān lǐjiě,`*v`cāozuò, shì `&v`de fǎn xiàng cāozuò, jí shìtú yóu zīyuán de yǐnyòng huòqǔ dào zīyuán de kǎobèi (rúguǒ zīyuán lèixíng shíxiànle `Copy`), huò suǒyǒuquán (zīyuán lèixíng méiyǒu shíxiàn `Copy`). Rust zhōng, běn cāozuò fú xíngwéi kěyǐ zhòng zài. Zhè yěshì Rust cāozuò fú de jīběn tèdiǎn. Běnshēn méiyǒu shé me tèbié de. ## Qiángzhì yǐn shì zhuǎnhuàn (coercion) `Deref`shénqí dì dìfāng bìng bùzài běnshēn `jiě yǐn `zhège yìyì shàng,Rust de shèjì zhě zài tā zhī shàng fùjiāle yīgè tèxìng:`Qiángzhì yǐn shì zhuǎnhuàn `, zhè cái shì tā shénqí zhī chù. Zhè zhǒng yǐn shì zhuǎnhuàn de guīzé wèi: Yīgè lèixíng wèi `T`de duìxiàng `foo`, rúguǒ `T: Deref<Target=U>`, nàme, xiāngguān `foo`de mǒu gè zhìnéng zhǐzhēn huò yǐnyòng (bǐrú `&foo`) zài yìngyòng de shíhòu huì zìdòng zhuǎnhuàn chéng `&U`. Cū kàn zhè tiáo guīzé, màosì yǒudiǎn lèisì yú `AsRef`, ér gēn `jiě yǐn `sìhū fēng mǎ niú bù xiāng jí. Shíjì lǐmiàn yǒuxiē xuánmiào zhī chù. Rust biānyì qì huì zài zuò `*v`cāozuò de shíhòu, zìdòng xiān bǎ `v`zuò yǐnyòng guī yī huà cāozuò, jí zhuǎnhuàn chéng nèibù tōngyòng yǐnyòng de xíngshì `&v`, zhěnggè biǎodá shì jiù biàn chéng `*&v`. Zhè lǐmiàn yǒu liǎng zhǒng qíngkuàng: 1. Bǎ qítā lèixíng de zhǐzhēn (bǐrú zài kù zhōng dìngyì de,`Box`, `Rc`, `Arc`, `Cow`děng), zhuǎn chéng nèibù biāozhǔn xíngshì `&v`; 2. Bǎ duōchóng `&` (bǐrú:`&&&&&&&V`), jiǎnhuà chéng `&v`(tōngguò chārù zúgòu shùliàng de `*`jìnxíng jiě yǐn). Suǒyǐ, tā shíjì shang zài jiě yǐnyòng zhīqián zuòle yīgè yǐnyòng de guī yī huà cāozuò. Wèishéme yào zhuǎn ne? Yīnwèi biānyì qì shèjì de nénglì shì, zhǐ nénggòu duì `&v`zhè zhǒng yǐnyòng jìnxíng jiě yǐnyòng. Qítā xíngshì de tā bù rènshí, suǒyǐ yào zuò yǐnyòng guī yī huà cāozuò. Shǐyòng yǐnyòng jìnxíng guòdù yěshì wèile nénggòu fángzhǐ bù bìyào de kǎobèi. Xiàmiàn jǔ yīxiē lìzi: ```Rust fn foo(s: &Str) { // borrow a string for a second } // String implements Deref<Target=str> let owned ="Hello".To_string(); // therefore, this works: Foo(&owned); ``` yīnwèi `String`shíxiànle `Deref<Target=str>`. ```Rust use std::Rc::Rc; fn foo(s: &Str) { // borrow a string for a second } // String implements Deref<Target=str> let owned ="Hello".To_string(); let counted = Rc::New(owned); // therefore, this works: Foo(&counted); ``` yīnwèi `Rc<T>`shíxiànle `Deref<Target=T>`. ```Rust fn foo(s: &[I32]) { // borrow a slice for a second } // Vec<T> implements Deref<Target=[T]> let owned = vec![1, 2, 3]; Foo(&owned); ``` yīnwèi `Vec<T>`shíxiànle `Deref<Target=[T]>`. ```Rust struct Foo; impl Foo { fn foo(&self) {println!("Foo"); } } let f =&&Foo; f.Foo(); (&f).Foo(); (&&f).Foo(); (&&&&&&&&f).Foo(); ``` shàngmiàn nà jǐ zhǒng hán shǔ de diàoyòng, xiàoguǒ shì yīyàng de. `Coercion`de shèjì, shì Rust zhōng jǐn yǒu de lèixíng yǐn shì zhuǎnhuàn, shèjì tā de mùdì, shì wèile jiǎnhuà chéngxù de shūxiě, ràng dàimǎ bù zhìyú guòyú fánsuǒ. Bǎ rén cóng wújìn de lèixíng xìjié zhōng jiětuō chūlái, ràng shù xiě Rust dàimǎ biàn chéng yī jiàn kuàilè de shìqíng.
Show more
1,722 / 5,000
Translation results
Translation result
#Deref

`Deref` is a trait of the `deref` operator `*`, such as `*v`.

It is generally understood that `*v` operation is the reverse operation of `&v`, that is, trying to obtain a copy of a resource from a resource reference (if the resource type implements `Copy`), or ownership (the resource type does not implement `Copy` ).

In Rust, the behavior of this operator can be overloaded. This is also a fundamental feature of Rust operators. Nothing special per se.

## Mandatory implicit conversion (coercion)

The magic of `Deref` is not in the sense of `dereferencing` itself. Rust designers have added a feature to it: `forced implicit conversion`, which is its magic.

The rules for this implicit conversion are:

An object `foo` of type `T`, if `T: Deref<Target=U>`, then a smart pointer or reference (such as `&foo`) related to `foo` will be automatically converted when it is applied into `&U`.

At first glance, this rule seems to be similar to `AsRef`, but it seems to have nothing to do with `dequote`. There are actually some mysteries in it.

When the Rust compiler performs `*v` operations, it will automatically perform reference normalization operations on `v`, that is, convert it into the internal universal reference form `&v`, and the entire expression will become `*&v`. There are two situations here:

1. Convert other types of pointers (such as `Box`, `Rc`, `Arc`, `Cow`, etc. defined in the library) to the internal standard form `&v`;
2. Simplify multiple `&` (for example: `&&&&&&&v`) into `&v` (dequote by inserting enough `*`).

So, it actually does a reference normalization before dereferencing.

Why do you want to turn? Because the ability of the compiler design is that it can only dereference the reference of `&v`. It does not recognize other forms, so it needs to perform reference normalization operations.

Using references for transitions is also to prevent unnecessary copies.

Here are some examples:

```rust
fn foo(s: &str) {
     // borrow a string for a second
}

// String implements Deref<Target=str>
let owned = "Hello".to_string();

// therefore, this works:
foo(&owned);
```

Because `String` implements `Deref<Target=str>`.

```rust
use std::rc::Rc;

fn foo(s: &str) {
     // borrow a string for a second
}

// String implements Deref<Target=str>
let owned = "Hello".to_string();
let counted = Rc::new(owned);

// therefore, this works:
foo(&counted);
```
Because `Rc<T>` implements `Deref<Target=T>`.

```rust
fn foo(s: &[i32]) {
     //borrow a slice for a second
}

// Vec<T> implements Deref<Target=[T]>
let owned = vec![1, 2, 3];

foo(&owned);
```

Because `Vec<T>` implements `Deref<Target=[T]>`.

```rust
struct Foo;

impl Foo {
    fn foo(&self) { println!("Foo"); }
}

let f = &&Foo;

f.foo();
(&f).foo();
(&&f).foo();
(&&&&&&&&f).foo();
```

The above function calls have the same effect.


The design of `coercion` is the only type implicit conversion in Rust. Its purpose is to simplify the writing of programs and make the code less complicated. Freeing people from endless type details makes writing Rust code a joy.
