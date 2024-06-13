/*
 *  Análise e Teste de Software
 *  João Saraiva
 *  2016-2017
 */

#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <powercap/powercap.h>
#include <raplcap/raplcap.h>
#include <math.h>
#include "rapl.h"
#include "sensors.h"
#include <pthread.h>
#include <unistd.h>
#include <signal.h>

#define TEMPERATURETHRESHOLD 24.75
#define VARIANCE 10
#define WHATTSCAP 25
#define MAX_STRING_LENGTH 500
#define MAX_COMMAND_LENGTH 500
#define MEASUREMENTS_FILE "measurements.csv"
#define TIME_OUT_LIMIT 60*1500


// Structure to hold function return value and timeout flag
struct ThreadData {
    int returnValue;
};

// Global structure to communicate between threads
struct ThreadData threadData;


long fib(int n) 
{
    if (n < 1)
        return 1;
    else
        return fib(n - 1) + fib(n - 2);
}

int initializeRapl(raplcap *rc){
    printf("Running with CAP\n");
    raplcap_limit rl_short, rl_long;
    uint32_t q, j, n, d;

    // initialize
    if (raplcap_init(rc))
    {// Signal handler for SIGALRM
        perror("raplcap_init");
        return -1;
    }

    // get the number of RAPL packages
    n = raplcap_get_num_packages(NULL);
    if (n == 0)
    {
        perror("raplcap_get_num_packages");
        return -1;
    }

    // assuming each package has the same number of die, only querying for package=0
    d = raplcap_get_num_die(rc, 0);
    if (d == 0)
    {
        perror("raplcap_get_num_die");
        raplcap_destroy(rc);
        return -1;
    }

    // for each package die, set a power cap of same limit for both long and short
    // a time window of 0 leaves the time window unchanged
    rl_long.watts = WHATTSCAP;
    rl_long.seconds = 0;
    rl_short.watts = WHATTSCAP;
    rl_short.seconds = 0;

    for (q = 0; q < n; q++)
    {
        for (j = 0; j < d; j++)
        {
            if (raplcap_pd_set_limits(rc, q, j, RAPLCAP_ZONE_PACKAGE, &rl_long, &rl_short))
            {
                perror("raplcap_pd_set_limits");
            }
        }
    }

    // for each package die, enable the power caps
    // this could be done before setting caps, at the risk of enabling unknown power cap values first
    for (q = 0; q < n; q++)
    {
        for (j = 0; j < d; j++)
        {
            if (raplcap_pd_set_zone_enabled(rc, q, j, RAPLCAP_ZONE_PACKAGE, 1))
            {
                perror("raplcap_pd_set_zone_enabled");
            }
        }
    }
    return 0;
}

// Function to measure memory usage of a command and return the result
int runCommand(const char *command) {
    char cmd[1024];
    int mem;
    sprintf(cmd, "{ /usr/bin/time -v %s > /dev/null; } 2>&1 | grep 'Maximum resident' | sed 's/[^0-9]\\+\\([0-9]\\+\\).*/\\1/'", command);
    
    FILE *fp2 = popen(cmd, "r");
    if (fp2 == NULL) {
        fprintf(stderr, "Error running command\n");
        exit(-1);
    }

    char buf[1024];
    while (fgets(buf, sizeof(buf), fp2) != NULL) {
    }

    int status = pclose(fp2);
    mem = atoi(buf);
    return mem;
}

void* programWorks(const char *command){
    FILE *fp2 = popen(command, "r");
    if (fp2 == NULL) {
        fprintf(stderr, "Error running program verification\n");
        return 0;
    }
    
    char buf[1024];
    while (fgets(buf, sizeof(buf), fp2) != NULL) {
    }
    int status = pclose(fp2);

    if (WIFEXITED(status)) {
        int exit_status = WEXITSTATUS(status);
        if (exit_status == 0) {
            printf("Command executed successfully\n");
            threadData.returnValue = 1;
        } else {
            printf("Command exited with an error status: %d\n", exit_status);
            threadData.returnValue = 0;
        }
    } else {
        printf("Command did not exit normally\n");
        threadData.returnValue = 0;
    }
    pthread_exit(NULL);
};

void writeErrorMessage(const char *language, const char *program, const char *errorMsg){
    FILE *fp;
    fp = fopen(MEASUREMENTS_FILE, "w");
    if (fp == NULL) {
        perror("Error opening measurements file");
        exit(-1);
    }

    fprintf(fp, "Language, Program, PowerLimit, Package, Core(s), GPU, DRAM, Time (ms), Temperature, Memory\n");
    fprintf(fp, "%s, %s, %d, %s, %s, %s, %s, %s, %s, %s\n", language, program, WHATTSCAP,
                        errorMsg,errorMsg,errorMsg,errorMsg,errorMsg,errorMsg,errorMsg);
    fclose(fp);
}

void runTesting(int ntimes, int core, const char *command, const char *language, const char *program){
    struct timespec ts;
    if (clock_gettime(CLOCK_REALTIME, &ts) == -1)
    {
        printf("unnespected behaviour\n");
        return;
    }

    ts.tv_sec += TIME_OUT_LIMIT;

    // Create a thread to execute the function
    pthread_t thread;
    pthread_create(&thread, NULL, programWorks, (void *)command);
    
    // Join the thread to ensure its completion and get the return value
    int state = pthread_timedjoin_np(thread, NULL, &ts);

    if (state!=0) {
        // Handle timeout: Function execution interrupted
        writeErrorMessage(language,program,"TimeOut");
        printf("Timeout: Function execution interrupted after %d seconds\n", TIME_OUT_LIMIT);
    } else {
        // Function executed within the specified timeout
        if(threadData.returnValue==1){ //Program works
            performMeasurements(command,language,program,ntimes,core);
        }
        else{
            writeErrorMessage(language,program,"ProgramError");
        }
    }
}

struct MeasureArgs {
    FILE *fp;
    int core;
    const char *command;
};

void* measure(struct MeasureArgs* args){
    clock_t begin, end;
    double time_spent;
    struct timeval tvb, tva;
    char str_temp[20];

    rapl_before(args->fp, args->core);
        
    begin = clock();
    gettimeofday(&tvb, 0);
        
    int mem = runCommand(args->command);
        
    end = clock();
    gettimeofday(&tva, 0);
    time_spent = ((tva.tv_sec - tvb.tv_sec) * 1000000 + tva.tv_usec - tvb.tv_usec) / 1000;
        
    rapl_after(args->fp, args->core);
    sprintf(str_temp, "%.1f", getTemperature());
    fprintf(args->fp, "%G, %s, %d\n", time_spent, str_temp, mem);
    fflush(args->fp);
}

// Function to perform the measurements and write to the measurements file
void performMeasurements(const char *command, const char *language, const char *program, int ntimes, int core) {
    struct timespec ts;

    FILE *fp;
    float temperature;

    rapl_init(core);

    fp = fopen(MEASUREMENTS_FILE, "w");
    if (fp == NULL) {
        perror("Error opening measurements file");
        exit(-1);
    }

    fprintf(fp, "Language, Program, PowerLimit, Package, Core(s), GPU, DRAM, Time (ms), Temperature, Memory\n");

    for (int i = 0; i < ntimes; i++) {
        fprintf(fp, "%s, %s, %d,", language, program, WHATTSCAP);
        temperature = getTemperature();

        while (temperature > TEMPERATURETHRESHOLD + VARIANCE) {
            printf("Sleeping\n");
            sleep(1);
            temperature = getTemperature();
        }

        if (clock_gettime(CLOCK_REALTIME, &ts) == -1){
            printf("unnespected behaviour\n");
            return;
        }

        ts.tv_sec += TIME_OUT_LIMIT;

        pthread_t thread;
        struct MeasureArgs* myArgs = (struct MeasureArgs*)malloc(sizeof(struct MeasureArgs));
        myArgs->command=command;
        myArgs->core=core;
        myArgs->fp=fp;
        pthread_create(&thread, NULL, measure, (void *)myArgs);
            
        // Join the thread to ensure its completion and get the return value
        int state = pthread_timedjoin_np(thread, NULL, &ts);
        if (state!=0) {
            // Handle timeout: Function execution interrupted
            writeErrorMessage(language,program,"TimeOut");
            printf("Timeout: Function execution interrupted after %d seconds\n", TIME_OUT_LIMIT);
            return;
        }
    }
    fclose(fp);
}



int main(int argc, char **argv) {
    char command[MAX_COMMAND_LENGTH];
    char language[MAX_STRING_LENGTH] = "";
    char program[MAX_STRING_LENGTH] = "";
    int ntimes = 1;
    int core = 0;
    raplcap rc;

    if (argc < 5) {
        printf("Usage: %s <command> <language> <program> <ntimes>\n", argv[0]);
        return -1;
    }

    strcpy(command, argv[1]);
    strcpy(language, argv[2]);
    strcpy(program, argv[3]);
    ntimes = atoi(argv[4]);

    printf("\n\n Command: %s | Language: %s | Program: %s | Ntimes %d \n\n", command, language, program, ntimes);

    fflush(stdout);

    if (WHATTSCAP != -1) {
        if(initializeRapl(&rc))
            return -1;
    }
    runTesting(ntimes,core, command, language, program);
    printf("\n\n END of PARAMETRIZED PROGRAM: \n");

    if (WHATTSCAP != -1) {
        if (raplcap_destroy(&rc)) {
            printf("Error destroying CAP\n");
            perror("raplcap_destroy");
        } else {
            printf("Successfully destroyed CAP\n");
        }
    }
    return 0;
    
}

