#include "sensors.h"

float getTemperature() {
    char command[100];
    char output[200];
    float temperature, mean_temp = 0.0;
    int count = 0;

    // Run sensors command and capture output
    int status = system("sensors > temp.txt");
    if (status != 0) {
        printf("Error executing command: sensors > temp.txt\n");
        return -1;
    }
    FILE* temp_file = fopen("temp.txt", "r");
        
    // Parse temperature from output
    while(fgets(output, 200, temp_file)) {
        if(strstr(output, "Core")) { //TODO mudar isto maybe
            sscanf(output, "Core %*d: +%f°C", &temperature);
            mean_temp += temperature;
            count++;
        }
    }

    if (count > 0)
        mean_temp /= count;
        
    fclose(temp_file);
    remove("temp.txt");
    printf("\nMean temperature in all cores is %.10f°C\n", mean_temp);

    return mean_temp;
}