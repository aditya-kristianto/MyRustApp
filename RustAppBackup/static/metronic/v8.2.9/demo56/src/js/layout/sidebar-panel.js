"use strict";

// Class definition
var KTAppSidebarPanel = function () {
	// Private variables
	var panel;

	var handleMenuScroll = function() {
		var menuActiveItem = panel.querySelector(".menu-link.active");

		if ( !menuActiveItem ) {
			return;
		} 

		if ( KTUtil.isVisibleInContainer(menuActiveItem, panel) === true) {
			return;
		}

		panel.scroll({
			top: KTUtil.getRelativeTopPosition(menuActiveItem, panel),
			behavior: 'smooth'
		});
	}

	// Public methods
	return {
		init: function () {
			// Elements
			panel = document.querySelector('#kt_sidebar_panel_body');
			
			if ( panel ) {
				handleMenuScroll();
			}
		}
	};
}();

// On document ready
KTUtil.onDOMContentLoaded(function () {
	KTAppSidebarPanel.init();
});