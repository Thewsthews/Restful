const { m } = require("framer-motion");

async function getBookRecommendations(params) {
    const response = await fetch('api/v1/books/recommendations',{
        headers:{
            'Authorization': 'Bearer ${token}',
            'Content-Type': 'application/json'
        },
        params: params
    });
    const data = await response.json();
    return data;
}

async function createBook(params) {

    const bookParams = {
        book: {
            title: params.title,
            author: params.author,
            genre: params.genre,
            description: params.description,
            price: params.price,
            rating: params.rating,
            keywords: params.keywords,
        }
    };
    
    const response =await fetch ('api/v1/books', {
        method: 'POST',
        headers:{
            'Authorization': 'Bearer ${token}',
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(params)
    });
    return await response.json();
}

try{
    const newBook = await createBook({
        title: "The Book",
        author: "Tommas",
        genre: "Technical",
        description: "A deep inscription",
        price: 20000,
        rating: 4.4,
        keywords: ["Book", "Technical", "Inscription"]
    });

    const recommendations = await getBookRecommendations({
        genre: "Technical",
        min_rating: 4.0
    });
} catch (error){
    console.error('Error:', error);
}
