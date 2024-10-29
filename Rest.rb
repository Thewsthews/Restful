class Book <ApplicationRecord
  validates: title, presence: true
  validates: author, presence: true

  belongs_to :category
  has_many :reviews
  has_many :orders
end

class User <ApplicationRecord
  has_many :reviews
  has_many :orders
  has_secured_passwords

  validates :email, presence: true, uniqueness: true
end

module Api
  module V1
    class BooksController < ApplicationRecord
      before_action :authenticate_request

      def index
        books = Book.all
        render json: books
      end

  def show
    book = Book.find(params[:id])
    render json: book
  rescue ActiveRecord::RecordNotFound
    render json: {error: 'Book not found'}, status: :not_found
  end
  def create
    book = Book.new(book_params)
    if book.save
      render json: bookm status: :created
    else
      render json: book.errors, status: :unprocessable_entity
    end
end

private


def book_params
  params.require(:book).permit(:title, :author, :genre, :price)
