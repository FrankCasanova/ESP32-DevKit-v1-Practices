/* memory.x */
MEMORY
{
  /* Instruction ROM in off‑chip flash (mapped into IROM) */
  irom_seg : ORIGIN = 0x40200000, LENGTH = 0x80000

  /* Data in off‑chip flash (mapped into DROM) */
  drom_seg : ORIGIN = 0x3F400000, LENGTH = 0x60000

  /* Internal instruction RAM */
  iram_seg : ORIGIN = 0x40080000, LENGTH = 0x20000

  /* Internal data RAM */
  dram_seg : ORIGIN = 0x3FFAE000, LENGTH = 0x52000

  /* RTC fast memory */
  rtc_fast_seg : ORIGIN = 0x50000000, LENGTH = 0x8000

  /* RTC slow memory */
  rtc_slow_seg : ORIGIN = 0x50002000, LENGTH = 0x2000
}
