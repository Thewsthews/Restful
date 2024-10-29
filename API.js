async function getBookRecommendations(params) {
    const response = await fetch('api/v1/books/recommendations',{
        headers:{
            'Authorization': 'Bearer ${token}',
            'Content-Type': 'application/json'
        }
    });
    const data = await response.json();
    return data;
}

async function createBook(params) {
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