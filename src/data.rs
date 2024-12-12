use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Macbook Air".to_string(),
            price: 1399.99,
            description: "Macbook Air".to_string(),
            image: "/macbookair.jpg".to_string()
        },
        Product {
            id: 2,
            name: "keyboard".to_string(),
            price: 36.99,
            description: "keyboard".to_string(),
            image: "/keyboard.jpg".to_string()
        },
        Product {
            id: 3,
            name: "Mouse".to_string(),
            price: 12.99,
            description: "mouse".to_string(),
            image: "/mouse.jpg".to_string()
        },
        Product {
            id: 4,
            name: "Iphone".to_string(),
            price: 2011.99,
            description: "Iphone 16".to_string(),
            image: "/iphone.jpg".to_string()
        },
        Product {
            id: 5,
            name: "Headphone".to_string(),
            price: 28.99,
            description: "headphone".to_string(),
            image: "/headphone.jpg".to_string()
        },
        Product {
            id: 6,
            name: "Monitor".to_string(),
            price: 114.99,
            description: "Monitor".to_string(),
            image: "/monitor.jpg".to_string()
        },
        Product {
            id: 7,
            name: "TV".to_string(),
            price: 1119.99,
            description: "TV".to_string(),
            image: "/TV.jpg".to_string()
        },
        Product {
            id: 8,
            name: "Video Game Controller".to_string(),
            price: 7.99,
            description: "Video Game Controller".to_string(),
            image: "/videogamecontroller.jpg".to_string()
        },
        Product {
            id: 9,
            name: "Dell Laptop".to_string(),
            price: 3.99,
            description: "Dell Laptop".to_string(),
            image: "/delllaptop.jpg".to_string()
        },
        Product {
            id: 10,
            name: "USB".to_string(),
            price: 5.99,
            description: "USB".to_string(),
            image: "/usb.jpg".to_string()
        }
    ]
}