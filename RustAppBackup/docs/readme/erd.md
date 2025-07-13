<a name="readme-top"></a>

# ERD

## Users Table
Schema : public

Table : users

| Column Name   | Type      | Length | Primary Key | Nullable |
|---------------|-----------|--------|-------------|----------| 
| user_id       | uuid      |        | Y           |          |        
| username      | varchar   | 50     |             |          |
| email         | varchar   | 255    |             |          |
| password_hash | varchar   | 60     |             |          |
| full_name     | varchar   | 100    |             |          |
| created_at    | timestamp |        |             |          |
| created_by    | uuid      |        |             |          |
| updated_at    | timestamp |        |             | Y        |    
| updated_by    | uuid      |        |             | Y        |     
| deleted_at    | timestamp |        |             | Y        |      
| deleted_by    | uuid      |        |             | Y        |

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Clients Table

Schema : public

Table : clients

| Column Name   | Type      | Length | Primary Key | Nullable |
|---------------|-----------|--------|-------------|----------| 
| client_id     | uuid      |        | Y           |          |        
| client_secret | varchar   | 50     |             |          |
| client_name   | varchar   | 255    |             |          |
| redirect_uri  | varchar   | 60     |             |          |
| created_at    | timestamp |        |             |          |
| created_by    | uuid      |        |             |          |
| updated_at    | timestamp |        |             | Y        |    
| updated_by    | uuid      |        |             | Y        |     
| deleted_at    | timestamp |        |             | Y        |      
| deleted_by    | uuid      |        |             | Y        |

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Access Token Table

Schema : public

Table : access_token

| Column Name | Type      | Length | Primary Key | Nullable |
|-------------|-----------|--------|-------------|----------| 
| token_id    | uuid      |        | Y           |          |        
| client_id   | varchar   | 50     |             |          |
| user_id     | varchar   | 255    |             |          |
| token_value | varchar   | 60     |             |          |
| expires_at  | timestamp |        |             |          |
| created_at  | timestamp |        |             |          |
| created_by  | uuid      |        |             |          |
| updated_at  | timestamp |        |             | Y        |    
| updated_by  | uuid      |        |             | Y        |     
| deleted_at  | timestamp |        |             | Y        |      
| deleted_by  | uuid      |        |             | Y        |

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Authorization Codes Table

Schema : public

Table : authorization_codes

| Column Name  | Type      | Length | Primary Key | Nullable |
|--------------|-----------|--------|-------------|----------| 
| code_id      | uuid      |        | Y           |          |        
| client_id    | varchar   | 50     |             |          |
| user_id      | varchar   | 255    |             |          |
| code_value   | varchar   | 60     |             |          |
| redirect_uri | varchar   | 255    |             |          |
| created_at   | timestamp |        |             |          |
| created_by   | uuid      |        |             |          |
| updated_at   | timestamp |        |             | Y        |    
| updated_by   | uuid      |        |             | Y        |     
| deleted_at   | timestamp |        |             | Y        |      
| deleted_by   | uuid      |        |             | Y        |

<p align="right">(<a href="#readme-top">back to top</a>)</p>

# Refresh Tokens Table

Schema : public

Table : refresh_tokens

| Column Name         | Type      | Length | Primary Key | Nullable |
|---------------------|-----------|--------|-------------|----------| 
| refresh_token_id    | uuid      |        | Y           |          |        
| client_id           | uuid      | 50     |             |          |
| user_id             | varchar   | 255    |             |          |
| refresh_token_value | varchar   | 60     |             |          |
| expires_at          | timestamp | 255    |             |          |
| created_at          | timestamp |        |             |          |
| created_by          | uuid      |        |             |          |
| updated_at          | timestamp |        |             | Y        |    
| updated_by          | uuid      |        |             | Y        |     
| deleted_at          | timestamp |        |             | Y        |      
| deleted_by          | uuid      |        |             | Y        |

 <p align="right">(<a href="#readme-top">back to top</a>)</p>