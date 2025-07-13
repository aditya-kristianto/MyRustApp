"use strict";

// Class definition
var KTMainPageMap = (function () {
  var selectedlocation;

  // Private functions
  var initMap = function (elementId) {
    // Check if Leaflet is included
    if (!L) {
      return;
    }

    // Define Map Location
    var leaflet = L.map(elementId, {
      center: [40.725, -73.985],
      zoom: 30,
    });

    // Init Leaflet Map. For more info check the plugin's documentation: https://leafletjs.com/
    L.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.webp", {
      attribution:
        '&copy; <a href="https://osm.org/copyright">OpenStreetMap</a> contributors',
    }).addTo(leaflet);

    // Set Geocoding
    var geocodeService;
    if (typeof L.esri.Geocoding === "undefined") {
      geocodeService = L.esri.geocodeService();
    } else {
      geocodeService = L.esri.Geocoding.geocodeService();
    }

    // Define Marker Layer
    var markerLayer = L.layerGroup().addTo(leaflet);

    // Set Custom SVG icon marker
    var leafletIcon = L.divIcon({
      html: `<i class="ki-solid ki-geolocation text-primary fs-3x"></span>`,
      bgPos: [10, 10],
      iconAnchor: [20, 37],
      popupAnchor: [0, -37],
      className: "leaflet-marker",
    });

    // Show current address
    L.marker([52.3609, 4.8593], { icon: leafletIcon })
      .addTo(markerLayer)
      .bindPopup("1054 XE Amsterdam, Netherlands", { closeButton: false })
      .openPopup();

    // Map onClick Action
    leaflet.on("click", function (e) {
      geocodeService
        .reverse()
        .latlng(e.latlng)
        .run(function (error, result) {
          if (error) {
            return;
          }
          markerLayer.clearLayers();
          selectedlocation = result.address.Match_addr;
          L.marker(result.latlng, { icon: leafletIcon })
            .addTo(markerLayer)
            .bindPopup(result.address.Match_addr, { closeButton: false })
            .openPopup();

          // Show popup confirmation. For more info check the plugin's official documentation: https://sweetalert2.github.io/
          Swal.fire({
            html:
              'Your selected - <b>"' +
              selectedlocation +
              " - " +
              result.latlng +
              '"</b>.',
            icon: "success",
            buttonsStyling: false,
            confirmButtonText: "Ok, got it!",
            customClass: {
              confirmButton: "btn btn-primary",
            },
          }).then(function (result) {
            // Confirmed
          });
        });
    });
  };

  return {
    // Public functions
    init: function () {
      initMap("kt_search_map");
    },
  };
})();

// On document ready
KTUtil.onDOMContentLoaded(function () {
  KTMainPageMap.init();
});
