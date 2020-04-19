#[cfg(not(feature = "std"))]
use alloc::{boxed::Box, vec::Vec};
use core::{fmt, marker::PhantomPinned, pin::Pin, slice};

/// An owned version of `ttf_parser::Font`
pub struct OwnedFont(Pin<Box<VecFont>>);

impl OwnedFont {
    pub fn try_from_vec(data: Vec<u8>, index: u32) -> Option<Self> {
        let inner = VecFont::try_from_vec(data, index)?;
        Some(Self(inner))
    }
}

impl fmt::Debug for OwnedFont {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OwnedFont")
    }
}

impl super::AsFontRef for OwnedFont {
    #[inline]
    fn font(&self) -> &ttf_parser::Font<'_> {
        self.0.inner_ref()
    }
}
impl super::AsFontRef for &OwnedFont {
    #[inline]
    fn font(&self) -> &ttf_parser::Font<'_> {
        self.0.inner_ref()
    }
}

struct VecFont {
    data: Vec<u8>,
    font: Option<ttf_parser::Font<'static>>,
    _pin: PhantomPinned,
}

impl VecFont {
    /// Creates an underlying font object from owned data.
    pub(crate) fn try_from_vec(data: Vec<u8>, index: u32) -> Option<Pin<Box<Self>>> {
        let font = Self {
            data,
            font: None,
            _pin: PhantomPinned,
        };
        let mut b = Box::pin(font);
        unsafe {
            // 'static lifetime is a lie, this data is owned, it has pseudo-self lifetime.
            let slice: &'static [u8] = slice::from_raw_parts(b.data.as_ptr(), b.data.len());
            let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut b);
            let mut_inner = mut_ref.get_unchecked_mut();
            mut_inner.font = Some(ttf_parser::Font::from_data(slice, index)?);
        }
        Some(b)
    }

    // Must not leak the fake 'static lifetime that we lied about earlier to the
    // compiler. Since the lifetime 'a will not outlive our owned data it's
    // safe to provide Font<'a>
    #[inline]
    pub(crate) fn inner_ref<'a>(self: &'a Pin<Box<Self>>) -> &'a ttf_parser::Font<'a> {
        match self.font.as_ref() {
            Some(f) => f,
            None => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}
