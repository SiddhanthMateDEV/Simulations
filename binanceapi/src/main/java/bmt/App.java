package bmt;

/**
 * Hello world!
 *
 */
// Import necessary libraries for HTTP connection, cryptography and I/O operations
import javax.crypto.Mac;
import javax.crypto.spec.SecretKeySpec;
import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.net.HttpURLConnection;
import java.net.URL;
import java.nio.charset.StandardCharsets;
import java.security.InvalidKeyException;
import javax.json.bind.Jsonb;
import javax.json.bind.JsonbBuilder;
import java.io.IOException;
import java.security.NoSuchAlgorithmException;
import java.time.Instant;
import java.time.temporal.ChronoUnit;


public class App {
    // Declare API URL and secret and API keys as constants
    private static final String API_URL = "https://vapi.binance.com";
    private static final String SECRET_KEY = "your_secret_key";
    private static final String API_KEY = "your_api_key";

    public static void main(String[] args) {
        try {
            // Get current timestamp in milliseconds
            long timestamp = System.currentTimeMillis();

            // Calculate timestamp for one month from now
            long timestampOneMonthFromNow = Instant.now().plus(1, ChronoUnit.MONTHS).toEpochMilli();

            // Prepare parameters
            String queryString = "timestamp=" + timestamp + "&recvWindow=" + timestampOneMonthFromNow;

            // Generate signature using HMAC-SHA256 algorithm
            String signature = hmacSHA256(queryString, SECRET_KEY);

            // Construct the URL for the API endpoint
            URL url = new URL(API_URL + "/vapi/v1/optionInfo?" + queryString + "&signature=" + signature);

            // Create an HttpURLConnection object from the URL
            HttpURLConnection connection = (HttpURLConnection) url.openConnection();

            // Set the request method to GET
            connection.setRequestMethod("GET");

            // Set the request headers
            connection.setRequestProperty("X-MBX-APIKEY", API_KEY);

            // Get the response code from the server
            int responseCode = connection.getResponseCode();

            // If the response code is 200, the request was successful
            if (responseCode == HttpURLConnection.HTTP_OK) {
                // Create a BufferedReader to read the response from the server
                BufferedReader in = new BufferedReader(new InputStreamReader(connection.getInputStream()));
                String inputLine;
                StringBuffer response = new StringBuffer();

                // Read the response line by line
                while ((inputLine = in.readLine()) != null) {
                    response.append(inputLine);
                }

                // Close the BufferedReader
                in.close();

                // Print the response
                // System.out.println(response.toString());
                Jsonb jsonbInstance = JsonbBuilder.create();

                String jsonString = jsonbInstance.toJson(response.toString());
                try(FileWriter writer = new FileWriter("options.json")){
                    writer.write(jsonString);
                }catch (IOException e){
                    e.printStackTrace();
                }
            } else {
                System.out.println("GET request not worked");
            }

        } catch (Exception e) {
            // Print any exceptions that occur
            e.printStackTrace();
        }
    }

    // Function to generate a HMAC-SHA256 signature
    private static String hmacSHA256(String data, String secret) throws NoSuchAlgorithmException, InvalidKeyException {
        // Initialize HMAC-SHA256 algorithm
        Mac sha256_HMAC = Mac.getInstance("HmacSHA256");
        SecretKeySpec secret_key = new SecretKeySpec(secret.getBytes(StandardCharsets.UTF_8), "HmacSHA256");
        sha256_HMAC.init(secret_key);

        // Compute the HMAC on input data bytes
        byte[] raw = sha256_HMAC.doFinal(data.getBytes(StandardCharsets.UTF_8));
        StringBuilder hexString = new StringBuilder();

        // Convert array of bytes into a hex string
        for (byte b : raw) {
            String hex = Integer.toHexString(0xff & b);
            if(hex.length() == 1) hexString.append('0');
            hexString.append(hex);
        }

        // Return the HMAC as a hex string
        return hexString.toString();
    }
}


