/*UW_BUILD:
{}
:UW_BUILD*/

#include <stdio.h>

#include "zip/zip.h"

// this is easier that changing the build system =)
#include "zip/zip.c"


int all_ok = 1;

void my_assert_impl(int value, const char* expression, const char* file, unsigned line) {
    all_ok = all_ok && value;
    
    printf("CHECK %20s:%-3d: %40s -> %s\n", file, line, expression, value ? "OK" : "FAIL");
}

#define my_assert(exp) my_assert_impl(!!(exp), #exp, __FILE__, __LINE__)

void mkfiles(void) {
    FILE* f;
    
    f = fopen("foO-2.1.TxT", "w");
    fwrite("aaaa\n", 1, 5, f);
    fclose(f);
    
    
    f = fopen("foo-2.2.TxT", "w");
    fwrite("bbbb\n", 1, 5, f);
    fclose(f);
    
    
    f = fopen("foO-2.3.txt", "w");
    fwrite("cccc\n", 1, 5, f);
    fclose(f);
}

int on_extract_entry(const char *filename, void *arg) {
    static int i = 0;
    int n = *(int *)arg;
    printf("Extracted: %s (%d of %d)\n", filename, ++i, n);

    return 0;
}

char buffer[1024];

char* freadall(const char* path) {
    FILE* f;
    int sz;
    
    memset(buffer, 0, sizeof(buffer));
    
    f = fopen(path, "r");
    
    fseek(f, 0L, SEEK_END);
    sz = ftell(f);
    rewind(f);
    
    fread(buffer, 1, sz, f);
    fclose(f);
    
    printf("Read %d bytes from %s:\n%s\n", sz, path, buffer);
    
    return buffer;
}

int main() {
    struct zip_t *zip;
    char* contents;
    
    printf("Creating the files..\n");
    mkfiles();
    
    printf("Creating the archive..\n");
    
    zip = zip_open("foo.zip", ZIP_DEFAULT_COMPRESSION_LEVEL, 'w');
    {
        zip_entry_open(zip, "foo-1.txt");
        {
            const char *buf = "Some data here...\0";
            zip_entry_write(zip, buf, strlen(buf));
        }
        zip_entry_close(zip);

        zip_entry_open(zip, "foo-2.txt");
        {
            // merge 3 files into one entry and compress them on-the-fly.
            zip_entry_fwrite(zip, "foo-2.1.txt");
            zip_entry_fwrite(zip, "foo-2.2.txt");
            zip_entry_fwrite(zip, "foo-2.3.txt");
        }
        zip_entry_close(zip);
    }
    zip_close(zip);

    {
        int arg = 2;
        zip_extract("foo.zip", "tmp", on_extract_entry, &arg);
    }
    
    contents = freadall("tmp/foo-1.txt");
    my_assert(!strcmp(contents, "Some data here..."));
    
    contents = freadall("tmp/foo-2.txt");
    my_assert(!strcmp(contents, "aaaa\nbbbb\ncccc\n"));
    
    my_assert(0);
    
    if (all_ok) {
        return 0;
    } else {
        return 1;
    }
}
