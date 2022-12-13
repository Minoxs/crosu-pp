#ifndef CROSU_PP
#define CROSU_PP

#include "stdint.h"
#include "stdbool.h"

typedef float f32;
typedef double f64;
typedef uint32_t u32;
typedef size_t usize;

const u32
				NF = 0b1,
				EZ = 0b10,
				TD = 0b100,
				HD = 0b1000,
				HR = 0b10000,
				DT = 0b100000,
				RX = 0b1000000,
				HT = 0b10000000,
				FL = 0b100000000,
				SO = 0b1000000000;

typedef struct {
		f64 aim;
		f64 speed;
		f64 flashlight;
		f64 sliderFactor;
		f64 speedNodeCount;
		f64 ar;
		f64 od;
		f64 hp;
		usize nCircles;
		usize nSliders;
		usize nSpinners;
		f64 stars;
		usize maxCombo;
} OsuDifficultyAttributes;

typedef struct {
		bool success;
		OsuDifficultyAttributes attr;
} OsuDiffResult;

typedef struct {
		usize combo;
		usize n300;
		usize n100;
		usize n050;
} Score;

OsuDiffResult GetOsuDifficultyAttributes(char* ptr, bool isFile, u32 mods);
f64 GetOsuPP(OsuDifficultyAttributes diff, Score score);

#endif // CROSU_PP
