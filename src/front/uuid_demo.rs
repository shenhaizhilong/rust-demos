

#[cfg(test)]
mod tests {
    use uuid::{NoContext, Timestamp, uuid, Uuid};

    #[test]
    fn test_uuid4() {
        let id = uuid::Uuid::new_v4();
        const ID: Uuid = uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8");

        println!("{id}, {ID}")
    }


    #[test]
    fn test_uuid7() {
        let ts = Timestamp::from_unix(NoContext, 1497624119, 1234);
        let id = uuid::Uuid::new_v7(ts);
        println!("{id}")
    }

}