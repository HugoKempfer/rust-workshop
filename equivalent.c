#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <stddef.h>
#include <stdlib.h>

typedef struct {
    char *name;
    void *payload;
} image_t;

typedef struct {
    char *name;
    char *mail;
    uint32_t age;
    image_t img;
    char *bio;
} user_t;

typedef enum {MESSAGE_CONTENT, IMAGE, SHARED_CONTACT} message_e;

typedef struct {
    message_e type;
    union {
        char *message_content;
        image_t image;
        user_t shared_contact;
    } payload;
} message_t;

image_t create_image(char *name, void *payload, size_t payload_size)
{
    return (image_t){strdup(name), strndup(payload, payload_size)};
}

message_t *fetch_message()
{
    message_t *msg = malloc(sizeof(*msg));

    if (!msg) {
        return NULL;
    }
    *msg = (message_t){MESSAGE_CONTENT, {.message_content = strdup("Damn")}};
    return msg;
}

int main(int argc, char *argv[])
{
    message_t *msg = fetch_message();

    if (!msg) {
        return 0;
    }

    switch (msg->type) {
        case MESSAGE_CONTENT:
            printf("Received new message -> %s\n", msg->payload.message_content);
            free(msg->payload.message_content);
            break;
        case IMAGE:
            printf("There is an image\n");
            break;
        case SHARED_CONTACT:
            printf("Shared contact name -> %s\n", msg->payload.shared_contact.name);
            break;
    }
    free(msg);
}
