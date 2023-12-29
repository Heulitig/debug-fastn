from flask import Flask, jsonify
import psycopg2

app = Flask(__name__)

# PostgreSQL connection parameters
db_params = {
    "host": "localhost",
    "port": 5432,
    "user": "testuser",
    "password": "testpassword",
    "database": "testdb",
}


def fetch_data():
    # Connect to the PostgreSQL database
    connection = psycopg2.connect(**db_params)

    try:
        # Create a cursor object to execute SQL queries
        with connection.cursor() as cursor:
            # Execute a query to fetch data from the 'test' table
            cursor.execute("SELECT * FROM test;")

            # Fetch all rows from the result set
            rows = cursor.fetchall()

            # Convert the results to a list of dictionaries
            data_list = [{"id": row[0], "data": row[1]} for row in rows]

    finally:
        # Close the database connection
        connection.close()

    return data_list


@app.route('/get_data', methods=['GET'])
def get_data():
    # Fetch data from the 'test' table
    data = fetch_data()

    # Return the data as JSON
    return jsonify(data)


if __name__ == '__main__':
    # Run the Flask application on port 5000
    app.run(port=5000)
