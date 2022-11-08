<style>
	* {
		font-family: Fira Code
  	}
</style>

```mermaid
flowchart LR;
	subgraph d00[ ]
		direction LR

		d000{{Start Button}}
		d001{{Pause Button}}
		d002{{Step Button}}
		d003{{Run Button}}

		d004[(State)]
		d005[(Run Input)]

		d006[/Sender\]

		d000 & d001 & d002 & d003 == Send ==> d006
		d000 & d001 -. Read & Write .-> d004
		d003 -. Read .-> d005
		d002 -. Read .-> d004
		d003 -. Read .-> d004
	end

	subgraph d01[ ]
		direction LR

		d010{{Speed Button}}

		d011[(Speed Input)]

		d012[/Sender\]

		d010 == Send ==> d012
		d010 -. Read .-> d011
	end

	subgraph d02[ ]
		direction LR

		d020{{Generate Button}}
		d021{{Clear Button}}

		d022[(Generate Input)]

		d023[/Sender\]

		d020 & d021 == Send ==> d023
		d020 -. Read .-> d022
	end

	subgraph d03[ ]
		direction LR

		d030{{Size Button}}

		d031[(Width Input)]
		d032[(Height Input)]
		d033[(Torus Input)]

		d034[/Sender\]

		d030 == Send ==> d034
		d030 -. Read .-> d031 & d032 & d033
	end

	subgraph d04[ ]
		direction LR

		d040{{Rules Button}}

		d041[(Rules Input)]

		d042[/Sender\]

		d040 == Send ==> d042
		d040 -. Read .-> d041
	end

	subgraph c1[ ];
		direction LR

		c10{{Visual Grid}}


		c14[(Clock)]
		c16[[Play]]
		c17[[Pause]]
		c18[[Run]]
		c19[[Change Speed]]
		c1c[[Change Size]]
		c1d[[Change Rules]]
		c1a[[Generate]]
		c1b[[Clear]]
		c1e[[Toggle]]

		c12[(Size)]
		c13[(Rules)]
		c11[(Data Grid)]
		c1f[\Receiver/]

		subgraph c15[C15: Step]
			direction TB

			d150[[Distribute]]
			d151[[Look Neighbors]]
			d152[[New State]]
			d153[[Combine]]

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
		c15 -. Read & Write .-> c11
		c18 & c14 -- Call --> c15
		c16 & c17 & c19 -. Write .-> c14
	end

	d006 ==> c1f
	d012 ==> c1f
	d023 ==> c1f
	d034 ==> c1f
	d042 ==> c1f
```
