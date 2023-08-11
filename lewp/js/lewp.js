class Lewp {
	constructor() {}

	init() {
		let all_module_scripts = document.querySelectorAll('script[data-lewp-type="component"]');
		all_module_scripts.forEach((module_script) => {
			let module_id = module_script.dataset.lewpId;
			if (module_id === undefined) {
				console.error({
					'message': 'Could not initialize module because it has no lewpId attached in its dataset',
					'domNode': module_script,
				})
				return;
			}
			let module_dom_nodes = document.querySelectorAll(
				'body [data-lewp-type="component"][class="' + module_id + '"]'
			);
			let module_url = new URL(module_script.src);
			import(module_url.pathname).then((module) => {
				if (!Object.keys(module).includes('init')) {
					console.debug({
						'message': 'No init method exported from script.',
						'domNode': module,
					})
					return;
				}
				module_dom_nodes.forEach((dom) => {
					module.init(dom);
				})
			})
		});
	}
}

const lewp = new Lewp();

document.addEventListener('DOMContentLoaded', () => {
	lewp.init();
});
