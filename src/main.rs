fn main() {
    println!("Hello, world!");
}


mod api {

    pub trait Board {
        type P: Place;
        type S: Square;
    
        fn get_square(&self, p: Self::P) -> &Self::S;
        fn set_square(&mut self, p: Self::P, s: &Self::S);
    }
    
    pub trait Place {
        type Rank;
        type File;
    
        fn rank(&self) -> Self::Rank;
        fn file(&self) -> Self::File;
    }
    
    pub trait Square {
        fn is_empty(&self) -> bool;
        fn is_occupied(&self) -> bool;
        fn is_black(&self) -> bool;
        fn is_white(&self) -> bool;
        fn is_pawn(&self) -> bool;
        fn is_rook(&self) -> bool;
        fn is_bishop(&self) -> bool;
        fn is_knight(&self) -> bool;
        fn is_queen(&self) -> bool;
        fn is_king(&self) -> bool;
        fn is_piece(&self) -> bool;
    }
    
    pub trait Squares {
        type P: Place;
    
        fn from(&self) -> &Self::P;
        fn to(&self) -> &Self::P;
    }

    pub trait Move : Squares {
        type S: Square;

        fn piece(&self) -> Self::S;
        fn taking(&self) -> Option<Self::S>;
    }
}

mod data {

    pub struct Square(u8);
    const BLACK: u8 = 1;
    const WHITE: u8 = 2;
    const PAWN: u8 = 4;
    const ROOK: u8 = 8;
    const BISHOP: u8 = 16;
    const KNIGHT: u8 = 32;
    const QUEEN: u8 = 64;
    const KING: u8 = 128;

    impl crate::api::Square for Square {

        fn is_empty(&self) -> bool { self.0 == 0 }
        fn is_occupied(&self) -> bool { self.0 != 0 }
        fn is_black(&self) -> bool { (self.0 & BLACK) != 0 }
        fn is_white(&self) -> bool { (self.0 & WHITE) != 0 }
        fn is_pawn(&self) -> bool { (self.0 & PAWN) != 0 }
        fn is_rook(&self) -> bool { (self.0 & ROOK) != 0 }
        fn is_bishop(&self) -> bool { (self.0 & BISHOP) != 0 }
        fn is_knight(&self) -> bool { (self.0 & KNIGHT) != 0 }
        fn is_queen(&self) -> bool { (self.0 & QUEEN) != 0 }
        fn is_king(&self) -> bool { (self.0 & KING) != 0 }
        fn is_piece(&self) -> bool { self.0 > PAWN }
    }

    pub struct Rank(u64);
    pub struct File(u64);

    pub struct Place(u64);

    impl crate::api::Place for Place {
        
        type Rank = Rank;
        type File = File;
        
        fn rank(&self) -> Rank { Rank(self.0 / 8) }
        fn file(&self) -> File { File(self.0 % 8) }
    }




}