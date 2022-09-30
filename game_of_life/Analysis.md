<style>
	* {
		font-family: Fira Code
  	}
</style>

```mermaid
flowchart LR;
	a([A: Game of Life])

	b0(B0: Input)
	b1(B1: Display)

	c00[C00: Play Input]
	c01[C01: Speed Input]
	c02[C02: Generate Input]
	c03[C03: Size Input]
	c04[C04: Rules Input]

	subgraph d00[ ]
		direction LR

		d000{{D000: Start Button}}
		d001{{D001: Pause Button}}
		d002{{D002: Step Button}}
		d003{{D003: Run Button}}

		d004[(D004: State)]
		d005[(D005: Run Input)]

		d006[/D006: Sender\]

		d000 & d001 & d002 & d003 == Send ==> d006
		d000 & d001 <-. Read & Write .-> d004
		d003 -. Read .-> d005
		d002 -. Read .-> d004
		d003 -. Read .-> d004
	end

	subgraph d01[ ]
		direction LR

		d010{{D010: Speed Button}}

		d011[(D011: Speed Input)]

		d012[/D012: Sender\]

		d010 == Send ==> d012
		d010 -. Read .-> d011
	end

	subgraph d02[ ]
		direction LR

		d020{{D020: Generate Button}}
		d021{{D021: Clear Button}}

		d022[(D022: Generate Input)]

		d023[/D023: Sender\]

		d020 & d021 == Send ==> d023
		d020 -. Read .-> d022
	end

	subgraph d03[ ]
		direction LR

		d030{{D030: Size Button}}

		d031[(D031: Width Input)]
		d032[(D032: Height Input)]
		d033[(D033: Torus Input)]

		d034[/D034: Sender\]

		d030 == Send ==> d034
		d030 -. Read .-> d031 & d032 & d033
	end

	subgraph d04[ ]
		direction LR

		d040{{D040: Rules Button}}

		d041[(D041: Rules Input)]

		d042[/D042: Sender\]

		d040 == Send ==> d042
		d040 -. Read .-> d041
	end

	subgraph c1[ ];
		direction LR

		c10{{C10: Visual Grid}}

		c11[(C11: Data Grid)]
		c12[(C12: Size)]
		c13[(C13: Rules)]
		c14[(C14: Clock)]

		c16[[C16: Play]]
		c17[[C17: Pause]]
		c18[[C18: Run]]
		c19[[C19: Change Speed]]
		c1a[[C1A: Generate]]
		c1b[[C1B: Clear]]
		c1c[[C1C: Change Size]]
		c1d[[C1D: Change Rules]]
		c1e[[C1E: Toggle]]

		c1f[\C1F: Receiver/]

		subgraph c15[C15: Step]
			direction TB

			d150[[D150: Distribute]]
			d151[[D151: Look Neighbors]]
			d152[[D152 New State]]
			d153[[D153: Combine]]

			d150 --> d151 --> d152 --> d153
		end

		c10 == Listen ==> c11
		c10 -- Call --> c1e
		c1e -. Read & Write .-> c11
		c1c -. Write .-> c12
		c1d -. Write .-> c13
		c12 & c13 <-- Compose --> c11
		c1f == Receive ==> c15 & c16 & c17 & c18 & c19
		c1f == Receive ==> c1a & c1b & c1c & c1d
		c1a & c1b -. Write .-> c11
		c15 <-. Read & Write .-> c11
		c18 & c14 -- Call --> c15
		c16 & c17 & c19 -. Write .-> c14
	end

	a --> b0 & b1

	b0 --> c00 & c01 & c02 & c03 & c04
	b1 ---> c1

	c00 --> d00
	c01 --> d01
	c02 --> d02
	c03 --> d03
	c04 --> d04
```
