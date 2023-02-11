mod BoardEdge{
    struct BoardEdge{
        id:String,
        nodes:[BoardSpace;2]
    }

    impl BoardEdge{
        fn new() -> Self{
            BoardEdge{
                id,
                nodes
            }
        }
        fn is_valid(&self)->bool{
            if (self.id.is_null()){
                false
            }
            for node in nodes.iter(){
                let edges = node.get_edges();
                
                if (!edges.contains(&self)){
                    false

                }
            }
        }

    }
}
