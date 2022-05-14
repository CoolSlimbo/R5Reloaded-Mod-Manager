<script>
	import {
		Col,
		Row,
		InputGroup,
		InputGroupText,
		Input,
		ListGroup,
		ListGroupItem,
		Button,
		ButtonGroup,
		Tooltip,
		FormGroup,
		Modal,
		ModalBody,
		ModalFooter,
		ModalHeader,
		Image
	} from "sveltestrap";
	import { invoke } from "@tauri-apps/api/tauri";

	export let selected = "Test";
	export let query = "";
	export let maps = [];
	export let mods = [];

	export let modManaAct = true;
	export let modMana;
	export let modBrow;

	export let grayIcon = "grayIcon.png";
	export let redIcon = "redIcon.png";

	let modsObject = [];
	export let modsObjectBet = [];

	let modScriptsObject = [];

	//console.log(modsObject);
	//console.log(modScriptsObject);

	export let launchOptions = "debug";
	export let dedicatedOption = false;

	export let logText = "";

	export let open = false;
	const toggle = () => (open = !open);

	function toggler(event) {
		for (let i = 0; i < event.target.parentElement.children.length; i++) {
			event.target.parentElement.children[i].classList.remove(
				"text-white"
			);
			event.target.parentElement.children[i].classList.add(
				"text-white-50"
			);
		}
		event.target.classList.remove("text-white-50");
		event.target.classList.add("text-white");

		logger("Selected: " + selected);
	}

	function modActive(event) {
		//console.log(event);

		if (modMana === event.target && event.target.classList.contains("active")) {
			for (let i = 0; i < event.target.parentElement.children.length; i++) {
				event.target.parentElement.children[i].classList.toggle("active");
			}
			
			modManaAct = !modManaAct;
		} else if (modMana !== event.target && !event.target.classList.contains("active")) {
			for (let i = 0; i < event.target.parentElement.children.length; i++) {
				event.target.parentElement.children[i].classList.toggle("active");
			}
			
			modManaAct = !modManaAct;
		}
	}

	function obtainModsInFolder() {
		invoke(`obtainModsInFolder`).then((message) => {
			mods = message;
			//console.log(mods);

			for (let i = 0; i < mods.length; i++) {
				if (mods[i].startsWith("Err. ")) {
					if (mods.startsWith("2", 5)) {
						logger("Unable to open file.");
					}
				} else {
					//console.log(mods)
					const obj = JSON.parse(mods[i]);
					console.log(obj);

					//console.log(obj.Name + obj.Description + obj.AppId + obj.enabled);

					if (obj.AppId === "" || obj.AppId === null || obj.AppId === undefined || obj.Name === "" || obj.Name === null || obj.Name === undefined || obj.Description === "" || obj.Description === null || obj.Description === undefined || obj.enabled === "" || obj.enabled === null || obj.enabled === undefined) {
						logger("Error. mod.json invalid. Missing AppId/Name/Description/Status");
						continue;
					}

					let mod = {
						[obj.AppId]: {
							name: obj.Name,
							description: obj.Description,
							version: obj.Version,
							enabled: obj.enabled,
							authors: obj.Authors,
							contacts: obj.Contacts,
						},
					};

					modsObject.push(mod);


					for (let i = 0; i < obj.CustomScripts.length; i++) {

						if (obj.CustomScripts[i].Path === "" || obj.CustomScripts[i].Path === null || obj.CustomScripts[i].Path === undefined || obj.CustomScripts[i].RunOn === "" || obj.CustomScripts[i].RunOn === null || obj.CustomScripts[i].RunOn === undefined) {
							logger("Error. mod.json invalid. Missing Custom Scripts infomation");
							continue;
						}

						let script = {
							[obj.AppId]: {
								path: obj.CustomScripts[i].Path,
								runOn: obj.CustomScripts[i].RunOn,
								clientCb: obj.CustomScripts[i].ClientCallback,
								serverCb: obj.CustomScripts[i].ServerCallback,
							},
						};

						modScriptsObject.push(script);
					}
				}
			}
			
			console.log(modsObject);
			modsObjectBet = modsObject;
			console.log(modScriptsObject);
			
			//console.log(Object.keys(modsObjectBet[0])[0]);
		});
	}

	function mapnames() {
		invoke(`getMapNames`).then((message) => {
			//logger(message);
			maps = message;

			maps.splice(maps.indexOf("frontend"), 1);
			maps.splice(maps.indexOf("englishserver_mp_common"), 1);

			//console.log(maps);
		});
	}

	function logger(text) {
		var d = new Date();
		var n = d.toLocaleTimeString();
		logText += "[" + n + "] " + text + "\n";
	}

	function launch() {
		let arg = "-debug";

		if (launchOptions === "debug" && dedicatedOption === false) {
			arg = "-debug";
		} else if (launchOptions === "debug" && dedicatedOption === true) {
			arg = "-dedicated_dev";
		} else if (launchOptions === "release" && dedicatedOption === false) {
			arg = "-release";
		} else if (launchOptions === "release" && dedicatedOption === true) {
			arg = "-dedicated";
		}

		invoke(`launchGame`, { arg: arg});
	}

	mapnames();
	obtainModsInFolder();
</script>

<svelte:head>
	<link
		rel="stylesheet"
		href="https://cdn.jsdelivr.net/npm/bootstrap@5.1.3/dist/css/bootstrap.min.css"
	/>
</svelte:head>

<main>
	<Row>
		<Col>
			<Row class="mb-1">
				<InputGroup>
					<InputGroupText class="bg-dark text-white"
						>Filter:</InputGroupText
					>
					<Input
						placeholder="Filter maps..."
						class="bg-dark text-white"
						bind:value={query}
					/>
				</InputGroup>
			</Row>
			<Row>
				<p class="text-white text-start fw-bold mb-0" id="maps">
					Maps:
				</p>
				<Tooltip target="maps" placement="bottom"
					><i>This feature doesn't work</i></Tooltip
				>
			</Row>
			<Row style="height: 83.5vh;" class="">
				<ListGroup class="p-0 border" flush style="border-radius: 20px">
					<ListGroupItem
						class="bg-dark border-light text-white"
						on:click={() => {
							selected = "menu";
						}}
						on:click={toggler}
						>Menu - Select to go here</ListGroupItem
					>
					{#each maps as map}
						{#if map.toLowerCase().includes(query.toLowerCase())}
							<ListGroupItem
								class="text-white-50 bg-dark border-light px-0"
								on:click={() => {
									selected = map.toLowerCase();
								}}
								on:click={toggler}
								>{map.toLowerCase()}</ListGroupItem
							>
						{/if}
					{/each}
				</ListGroup>
			</Row>
		</Col>
		<Col xs="8" class="mx-1">
			<Row>
				<ButtonGroup>
					<Button
						size="sm"
						active
						on:click={modActive}
						class="text-white" bind:this={modMana}>Mod Manager</Button
					>
					<Button size="sm" on:click={modActive} class="text-white" BIND:THIS={modBrow}
						>Mod Browser</Button
					>
				</ButtonGroup>
			</Row>
			<Row
				class="mb-1 mt-1 border overflow-scroll rounded"
				style="height: 55vh;">

				{#if modManaAct === true}
					<ListGroup flush class="p-0">
						{#each Object.keys(modsObjectBet) as mod, i}
							<ListGroupItem class="text-white border-light p-1" style="background-color: #212529;">
								<Row>
									<Col xs="2" class="p-0 m-0">
										<Image src={redIcon} style="width: 4.2em;"/>
									</Col>
									<Col>
										<Row>
											<h4 class="text-start p-0">{Object.values(modsObjectBet[i])[0].name}</h4>
										</Row>
										<Row>
											<p class="p-0 fst-italic text-start m-0">{Object.values(modsObjectBet[i])[0].description}</p>
										</Row>
									</Col>
									<Col xs="3">
										<Row>
											<Button class="p-0 m-0">Toggle</Button>
										</Row>
										<Row>
											<p class="fst-light fst-italic pb-0 pt-2 mb-0" id={Object.keys(modsObjectBet[i])}>Info</p>
											<Tooltip target={Object.keys(modsObjectBet[i])} placement="bottom">
												<b>VERSION:</b> <i>{Object.values(modsObjectBet[i])[0].version}</i> <br>
												<b>AUTHOR:</b> <i>{Object.values(modsObjectBet[i])[0].authors}</i> <br>
												<b>CONTACTS:</b> <i>{Object.values(modsObjectBet[i])[0].contacts}</i> <br>
												<b>APPID:</b> <i>{Object.keys(modsObjectBet[i])}</i> <br>
											</Tooltip>
										</Row>
									</Col>
								</Row>
							</ListGroupItem>
						{/each}
					</ListGroup>
				{:else if modManaAct === false}
					<p>I need to add stuff lol</p>
				{/if}

				
			</Row>
			<Row>
				<Col
					class="me-1 border overflow-auto logger ps-0"
					style="height: 33vh; word-wrap: break-word;"
				>
					<code
						><p class="text-white text-start fs-6 ps-1" style="">
							{logText}
						</p></code
					>
				</Col>
				<Col class="ps-2 border" sm="3" style="height: 33vh">
					<FormGroup class="mt-2">
						<Input
							id="r1"
							type="radio"
							bind:group={launchOptions}
							value="debug"
							label="Debug"
							class="text-white text-start mb-2 ms-1"
						/>
						<Input
							id="r2"
							type="radio"
							bind:group={launchOptions}
							value="release"
							label="Release"
							class="text-white text-start mt-2 ms-1"
						/>
					</FormGroup>
					<FormGroup class="mt-2">
						<Input
							id="c1"
							type="checkbox"
							label="Dedicated?"
							class="text-white text-start ms-1"
							bind:checked={dedicatedOption}
						/>
					</FormGroup>
					<Button size="lg" class="mt-3" on:click={launch}
						>LAUNCH</Button
					>
				</Col>
			</Row>
		</Col>
	</Row>
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
		white-space: pre-line;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
