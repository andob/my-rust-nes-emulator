                      The ultimate NES CPU test ROM
                      -----------------------------


V1.00 - 09/06/04

By: Kevin Horton


---


What it is:

This here is a pretty much all inclusive test suite for a NES CPU.
It was designed to test almost every combination of flags, instructions,
and registers.  Some of these tests are very difficult, and so far,
Nesten and Nesticle failed it.  Nintendulator passes, as does a real
NES (naturally).  I haven't tested it with any more emualtors yet.

I attempted to check the states of all flags after most instructions. 
For example, CPY and CMP shouldn't affect the overflow flag, while SBC
and ADC should.  Likewise, all forms of wrapping ARE tested for- zeropage
wrapping being the tests most emulators fail.  

i.e.

LDA #001h
LDA 0ffh,X   ;indexed zeropage read

should read the byte from 000h... NOT 0100h.   This is because zeropage
instructions cannot cross a page boundary.

---

How to work it good:

Simply run the .NES ROM on your emulator of choice.  You can select a single
test to run, or you can run ALL tests in sequence by selecting the
appropriate option.

Pressing Select will change pages and allow testing "invalid" opcodes.
Be aware that these will crash alot of emulators <cough>Nesten<cough>.

Once a test completes, the result will be "OK" if the test passes, or a
2 digit hex number which indicates a failure of some kind.  A list is
provided below for the failure and its cause.  For a more detailed reason
for the failure, you should check out the .ASM file included with this
document.

If the entire page of tests succeeds, "OK" will be displayed next to the
first entry on the page.  If one or more tests fails, "Er" will be displayed
instead.

---

NSF player testing:

This ROM is set up to be usable inside an NSF player.  It outputs the 
results of the test audially.  <to be finished>

---

Emulator authors:

This test program, when run on "automation", (i.e. set your program counter
to 0c000h) will perform all tests in sequence and shove the results of
the tests into locations 02h and 03h.  

---

Final notes:

The hex numbers shown on the screen (or stored in the above mentioned
memory locations) are of the LAST test that failed in the group tested. 
This means, there could be multiple failures in one or more groups.  This
wasn't the best solution, but since there are close to 400 tests performed,
any other way wouldn't have had acceptable memory usage.  So long as your
emulator bugs are fixed and the numbers are getting smaller, you're doing
good :-)