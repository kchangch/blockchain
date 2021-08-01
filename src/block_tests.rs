#[cfg(test)]
mod block_tests {
    use crate::block::Block;
    #[test]
    fn example() {
        // Create blocks
        let mut b0 = Block::initial(19);
        b0.set_proof(87745);
        let mut b1 = Block::next(&b0, String::from("hash example 1234"));
        b1.set_proof(1407891);


        // Test for valid proof
        let mut res = b0.is_valid();
        println!("Block b0 is valid proof 87745: {:?}", res);
        res = b1.is_valid();
        println!("Block b1 is valid proof 1407891: {:?}", res);
        b1.set_proof(346082);
        res = b1.is_valid();
        println!("Block b1 is valid proof 346082: {:?}", res);

        // Mining test
        let mut b0 = Block::initial(7);
        b0.mine(1);
        println!("{}", b0.hash_string());
        println!("{:02x}", b0.hash());
        let mut b1 = Block::next(&b0, String::from("this is an interesting message"));
        b1.mine(1);
        println!("{}", b1.hash_string());
        println!("{:02x}", b1.hash());
        let mut b2 = Block::next(&b1, String::from("this is not interesting"));
        b2.mine(1);
        println!("{}", b2.hash_string());
        println!("{:02x}", b2.hash());

        let mut b2 = Block::initial(20);
        b2.mine(10);
        println!("{}", b2.hash_string());
        println!("{:02x}", b2.hash());
        let mut b3 = Block::next(&b2, String::from("more workers"));
        b3.mine(10);
        println!("{}", b3.hash_string());
        println!("{:02x}", b3.hash());
        let mut b4 = Block::next(&b3, String::from("more difficult"));
        b4.mine(10);
        println!("{}", b4.hash_string());
        println!("{:02x}", b4.hash());

        let mut b5 = Block::initial(15);
        b5.mine(15);
        println!("{}", b5.hash_string());
        println!("{:02x}", b5.hash());
        let mut b6 = Block::next(&b5, String::from("hello world"));
        b6.mine(15);
        println!("{}", b6.hash_string());
        println!("{:02x}", b6.hash());

    }
}
