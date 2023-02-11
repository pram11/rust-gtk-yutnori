pub mod BoardSpace{

    struct BoardSpace{
        id:String,
        pos_x:usize,
        pos_y:usize,
        connected:Vec<BoardSpace>,
        edges:Vec<BoardEdge>,
        next:Option<BoardSpace>,
        shortcut:Option<BoardSpace>,
        is_start:bool,
        is_end:bool
    }

    impl BoardSpace{
        fn new() -> Self{
            BoardSpace{
                id,
                pos_x,
                pos_y,
                connected:Vec<BoardSpace>::new(),
                edges:Vec<BoardEdge>::new(),
                next:Option<BoardSpace>::new(),
                shortcut:Option<BoardSpace>::new(),
                is_start,
                is_end
            }

        }

        fn is_valid(&self) -> bool{
            if (self.id.is_null()){
                false
            }
            if (self.next.is_null()){
                if (!self.is_last()){
                false
                }
            }
            true
        }
        
        fn get_edges(&self)-> Vec<BoardEdge>{
            edges
        }
    }

}


