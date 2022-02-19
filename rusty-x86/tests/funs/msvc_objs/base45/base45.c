#include <ctype.h> 	// for size_t
#include <string.h> 	// for strlen()

/*                  Table 1: The Base 45 Alphabet
   Value Encoding  Value Encoding  Value Encoding  Value Encoding
      00 0            12 C            24 O            36 Space
      01 1            13 D            25 P            37 $
      02 2            14 E            26 Q            38 %
      03 3            15 F            27 R            39 *
      04 4            16 G            28 S            40 +
      05 5            17 H            29 T            41 -
      06 6            18 I            30 U            42 .
      07 7            19 J            31 V            43 /
      08 8            20 K            32 W            44 :
      09 9            21 L            33 X
      10 A            22 M            34 Y
      11 B            23 N            35 Z
*/

static const char BASE45_CHARSET[] = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ $%*+-./:";

static char _C2I[256] = {
	255,255,255,255, 255,255,255,255, 255,255,255,255, 255,255,255,255,
	255,255,255,255, 255,255,255,255, 255,255,255,255, 255,255,255,255,
	36, 255,255,255,  37, 38,255,255, 255,255, 39, 40, 255, 41, 42, 43,
         0,   1,  2,  3,   4,  5,  6,  7,   8,  9, 44,255, 255,255,255,255, 

        255, 10, 11, 12,  13, 14, 15, 16,  17, 18, 19, 20,  21, 22, 23, 24, /* uppercase */
         25, 26, 27, 28,  29, 30, 31, 32,  33, 34, 35, 35, 255,255,255,255,
        255, 10, 11, 12,  13, 14, 15, 16,  17, 18, 19, 20,  21, 22, 23, 24, /* lowercase */
         25, 26, 27, 28,  29, 30, 31, 32,  33, 34, 35, 35, 255,255,255,255,

	255,255,255,255, 255,255,255,255, 255,255,255,255, 255,255,255,255, 
	255,255,255,255, 255,255,255,255, 255,255,255,255, 255,255,255,255,
	255,255,255,255, 255,255,255,255, 255,255,255,255, 255,255,255,255,
	255,255,255,255, 255,255,255,255, 255,255,255,255, 255,255,255,255,

	255,255,255,255, 255,255,255,255, 255,255,255,255, 255,255,255,255,
	255,255,255,255, 255,255,255,255, 255,255,255,255, 255,255,255,255,
	255,255,255,255, 255,255,255,255, 255,255,255,255, 255,255,255,255,
	255,255,255,255, 255,255,255,255, 255,255,255,255, 255,255,255,255,
};

int base45_encode(char * dst, size_t *_max_dst_len, const unsigned char * src, size_t src_len) {
  size_t out_len = 0, max_dst_len;
  int i, x;
  unsigned char e, d, c;

  max_dst_len = _max_dst_len ? *_max_dst_len : src_len * 4;
  

  for(i = 0; i < src_len; i+=2) {
     if (src_len - i > 1) {
        x = ((src[i])<<8) + src[i+1];

        e = x / (45 * 45);
        x %= 45 * 45;
        d = x / 45;
        c = x % 45;

	if (out_len < max_dst_len && dst)
		dst[ out_len ] = BASE45_CHARSET[c];
	out_len++;
	if (out_len < max_dst_len && dst)
		dst[ out_len ] = BASE45_CHARSET[d];
	out_len++;
	if (out_len < max_dst_len && dst)
		dst[ out_len ] = BASE45_CHARSET[e];
	out_len++;
     } else {
        x = src[i];
        d = x / 45;
        c = x % 45;

	if (out_len < max_dst_len && dst)
		dst[ out_len ] = BASE45_CHARSET[c];
	out_len++;

	if (out_len < max_dst_len && dst)
		dst[ out_len ] = BASE45_CHARSET[d];
	out_len++;
     }
  }
  /* Same non guarantee as strncpy et.al. */
  if (out_len < max_dst_len && dst)
	dst[ out_len ] = 0;

  if (_max_dst_len)
	*_max_dst_len = out_len;
  return 0;
}

int base45_decode(unsigned char * dst, size_t * _max_dst_len, const char * src, size_t src_len) {
  size_t out_len = 0, max_dst_len;
  int i;
  max_dst_len = _max_dst_len  ? *_max_dst_len : src_len;

  if (dst == NULL && _max_dst_len == NULL)
	return -2;

  if (src == NULL)
	return -2;

  if (src_len == 0)
	src_len = strlen(src);

  for(i = 0; i < src_len; i+=3) {
     int x,a,b;

     if (src_len - i < 2) 
	return -1;

     if ((255 == (a = _C2I[src[i]])) || (255 == (b = _C2I[src[i+1]]))) 
	return -1;

     x = a + 45 * b;

     if (src_len - i >= 3) {
        if (255 == (a = _C2I[src[i+2]])) 
	    return -1;

        x += a * 45 * 45;

        if (out_len < max_dst_len && dst)
        	dst[out_len] = x / 256;
        out_len++;
	x %= 256;
    };

    if (out_len < max_dst_len && dst)
        dst[out_len] = x;

    out_len++;
  };
  if (_max_dst_len)
	*_max_dst_len = out_len;

  return 0;
} 

#define BUFSZ 1024

char rawbytes [BUFSZ];
char encoded  [BUFSZ];
char decoded  [BUFSZ];
char reencoded[BUFSZ];

size_t encoded_size   = BUFSZ;
size_t decoded_size   = BUFSZ;
size_t reencoded_size = BUFSZ;

int encode_ret;
int decode_ret;
int reencode_ret;

void start(int seed, int ln) {
    int i;
    
    seed = 0;
    ln = 0;
    
    if (ln > BUFSZ / 4) {
        ln = BUFSZ / 4;
    }
    
    for (i = 0; i < ln; i++) {
        rawbytes[i] = seed % 256;
        // https://en.wikipedia.org/wiki/Linear_congruential_generator#Parameters_in_common_use
        // Borland C++
        seed = 22695477 * seed + 1;
    }
    
    encode_ret   = base45_encode(encoded, &encoded_size, rawbytes, ln);
    decode_ret   = base45_decode(decoded, &decoded_size, encoded, encoded_size);
    reencode_ret = base45_encode(reencoded, &reencoded_size, decoded, decoded_size);
}
