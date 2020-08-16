MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* TODO Adjust these memory regions to match your device memory layout */
  FLASH : ORIGIN = 0x08000000, LENGTH = 128K /* STM32F103RBT6 */
  RAM : ORIGIN = 0x20000000, LENGTH = 20K /* STM32F103RBT6 */
}