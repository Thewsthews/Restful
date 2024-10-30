use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use serde_json::{Serialize, Deserialize};

#derive[(Serialize, Deserialize)]
struct Book{
    id: i32,
    title: String,
    author: String,
    genre: String,
    rating: f32
}

#derive[(Serialize, Deserialize)]
struct RecommendationRequest{
    user_id: i32,
    preferred_genres: Vec<String>
}

#get[("/recommendations/{user_id}")]
async fn get_recommendations(path: web::Path<i32>) -> HttpResponse{
    let recommendations = vec![
        Book {
        id: 1,
        titles: String::from("The PopEye"),
        author: String::from("Tom Thompson"),
        genre: String::from("Animation"),
        rating: 4.9
        }
    ];
    HttpResponse::Ok().json(recommendations)
}

#[post("/search")]
async fn search_books(query: web::Json<SearchQuery>) -> HttpResponse{
    let books = get_mock_books();
    let search_term - query.query.to_lowercase();
    let mut seacrch: Vec<(Book, f32)> = books
    .iter()
    .filter_map(|book|{
        let relevance_score = calculate_relevance(book, &search_term, &query.filters);
        if relevance_score > 0.0 {
            Some((book.clone(), relevance_score))
        }else{
            None
        }
    })
    .collect();

    search_results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    let final_results: Vec<Book> = search_results.into_iter()
    .map(|(book, _score)| book)
    .collect();
    HttpResponse::Ok().json(final_results)
}

fn calculate_relevance(book: &Book, search_term: &str, filters: &Options<SearchFilters>) -> f32{
    let mut score = 0.0;

    if book.titles.to_lowercase().contains(search_term){
        score += 10.0;
        
        if book.title.to_lowercase() == search_term{
            score += 5.0;
        }
    }

    if book.author.to_lowercase().contains(search_term){
        score += 8.0;
    }

    if book.description.to_lowercase().contains(search_term){
        score += 5.0;
    }
    let keyword_matches = book.keywords
    .iter()
    .filter(|k| k.to_lowercase().contains(search_term))
    .count();
score += (keyword_matches as f32) * 3.0;

if let Some(filters) = filters{
    if let Some(genre) = &filters.genre{
        if book.genre.to_lowercase() != genre.to_lowercase(){
            return 0.0;
        }
    }
    if let Some(min_rating) = filters.min_rating{
        if book.rating < min_rating{
            return 0.0;
        }
    }
    if let Some(author) = &filters.author{
        if !book.author.to_lowercase().contains(&author.to_lowercase()){
            return 0.0;
        }
    }
}

score *= (1.0 + (book.rating - 3.0)*0.1);

score
}

fn get_mock_books() -> Vec<Book>[
    vec![Book{
        id: 1,
            title: "The Rust Programming Language".to_string(),
            author: "Steve Klabnik".to_string(),
            genre: "Technical".to_string(),
            description: "Comprehensive guide to Rust programming".to_string(),
            rating: 4.8,
            keywords: vec!["programming".to_string(), "rust".to_string(), "systems".to_string()]
    }]
]