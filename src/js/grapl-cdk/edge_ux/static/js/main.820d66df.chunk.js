(this["webpackJsonpgrapl-fe"]=this["webpackJsonpgrapl-fe"]||[]).push([[0],{105:function(e,n,t){},144:function(e,n,t){e.exports=t(232)},149:function(e,n,t){},230:function(e,n){},232:function(e,n,t){"use strict";t.r(n);var a=t(0),r=t.n(a),c=t(15),o=t.n(c),s=(t(149),t(128)),i=t(280),l=t(23),u=t(10),d=(t(105),t(13)),f=t.n(d),p=t(20),m=t(63),h=function(){return"http://"+window.location.hostname+":5000/"},g="http://"+window.location.hostname+":8900/",v=function(){var e=Object(p.a)(f.a.mark((function e(){var n,t;return f.a.wrap((function(e){for(;;)switch(e.prev=e.next){case 0:return e.next=2,fetch("".concat(g,"checkLogin"),{method:"get",credentials:"include"});case 2:return n=e.sent,e.next=5,n.json();case 5:return t=e.sent,e.abrupt("return","True"===t.success);case 7:case"end":return e.stop()}}),e)})));return function(){return e.apply(this,arguments)}}(),b=function(e){return r.a.createElement("div",{className:"backgroundImage"},r.a.createElement("div",{className:"grapl"}," Grapl "),r.a.createElement("div",{className:"formContainer"},r.a.createElement(m.c,{initialValues:{userName:"",password:""},onSubmit:function(){var n=Object(p.a)(f.a.mark((function n(t){var a;return f.a.wrap((function(n){for(;;)switch(n.prev=n.next){case 0:return n.next=2,E(t.userName,t.password);case 2:a=n.sent,O(t.userName,a)?(e.loginSuccess(),console.log("Logged in")):console.warn("Login failed!");case 5:case"end":return n.stop()}}),n)})));return function(e){return n.apply(this,arguments)}}()},r.a.createElement(m.b,null,r.a.createElement(m.a,{name:"userName",type:"text",placeholder:"Username"})," ",r.a.createElement("br",null),r.a.createElement(m.a,{name:"password",type:"password",placeholder:"Password"})," ",r.a.createElement("br",null),r.a.createElement("button",{className:"s ubmitBtn",type:"submit"},"Submit")))))};function y(e){return j.apply(this,arguments)}function j(){return(j=Object(p.a)(f.a.mark((function e(n){var t,a,r;return f.a.wrap((function(e){for(;;)switch(e.prev=e.next){case 0:return t=(new TextEncoder).encode(n),e.next=3,crypto.subtle.digest("SHA-256",t);case 3:return a=e.sent,r=Array.from(new Uint8Array(a)),e.abrupt("return",r.map((function(e){return("00"+e.toString(16)).slice(-2)})).join(""));case 6:case"end":return e.stop()}}),e)})))).apply(this,arguments)}var E=function(){var e=Object(p.a)(f.a.mark((function e(n,t){var a,r;return f.a.wrap((function(e){for(;;)switch(e.prev=e.next){case 0:return"f1dafbdcab924862a198deaa5b6bae29aef7f2a442f841da975f1c515529d254",e.next=3,y(t+"f1dafbdcab924862a198deaa5b6bae29aef7f2a442f841da975f1c515529d254"+n);case 3:a=e.sent,r=0;case 5:if(!(r<5e3)){e.next=12;break}return e.next=8,y(a);case 8:a=e.sent;case 9:r++,e.next=5;break;case 12:return e.abrupt("return",a);case 13:case"end":return e.stop()}}),e)})));return function(n,t){return e.apply(this,arguments)}}(),O=function(){var e=Object(p.a)(f.a.mark((function e(n,t){var a,r;return f.a.wrap((function(e){for(;;)switch(e.prev=e.next){case 0:return e.prev=0,e.next=3,fetch("".concat(g,"login"),{method:"post",body:JSON.stringify({username:n,password:t}),headers:{"Content-Type":"application/json"},credentials:"include"});case 3:return a=e.sent,e.next=6,a.json();case 6:return r=e.sent,e.abrupt("return","True"===r.success);case 10:return e.prev=10,e.t0=e.catch(0),console.log(e.t0),e.abrupt("return",!1);case 14:case"end":return e.stop()}}),e,null,[[0,10]])})));return function(n,t){return e.apply(this,arguments)}}(),k=t(7),x=t(28),w=t(59),_=t(127),N=t(83),S=t(9),L=function(e,n){var t=e;"object"===typeof e&&(t=e.risk);var a=n.map((function(e){return e||0})).sort((function(e,n){return e-n}));if(void 0===t||0===t||0===a.length)return 0;var r,c=0,o=Object(S.a)(a);try{for(o.s();!(r=o.n()).done;){t>=r.value&&(c+=1)}}catch(s){o.e(s)}finally{o.f()}return Math.floor(c/a.length*100)},P=function(e,n){var t,a=Object(S.a)(n||[]);try{for(a.s();!(t=a.n()).done;){var r=t.value;if(r.id===e)return r}}catch(c){a.e(c)}finally{a.f()}return null},A=function(e,n){var t=function(e,n){var t=P(e.source.name,n.nodes),a=P(e.target.name,n.nodes);if(!t||!a)return console.error("Missing srcNode/dstNode",t,e,a),0;var r=t.risk||0,c=a.risk||0;return Math.round((r+c)/2)}(e,n),a=Object(x.a)(n.nodes).map((function(e){return e.risk}));return L(t,a)},F=function(e){var n=0;e+="x";for(var t=parseInt(65745979961613.07),a=0;a<e.length;a++)n>t&&(n=parseInt(n/137)),n=131*n+e.charCodeAt(a);return n},C=function(e,n){return"Process"===e.nodeType?[31,185,128]:"File"===e.nodeType?[177,93,255]:n.rgb(e.nodeType)},I=function(e){return"hsl(".concat(40*(100-e)/100,", 100%, 50%)")},M=function(e,n){var t=A(e,n);return 0===t?"white":I(t)},D=function(e,n){var t=new Set;R(e,(function(a,r){n(e,a,r),function e(n,t,a){R(n,(function(r,c){t.has(n.uid+r+c.uid)||(t.add(n.uid+r+c.uid),a(n,r,c),e(c,t,a))}))}(r,t,n)}))},T=function(e,n){for(var t in e)if(Object.prototype.hasOwnProperty.call(e,t)){var a=e[t];if(Array.isArray(a)){var r,c=Object(S.a)(a);try{for(c.s();!(r=c.n()).done;){var o=r.value;void 0!==o.uid&&n(t,o)}}catch(s){c.e(s)}finally{c.f()}}else a&&void 0!==a.uid&&n(t,a)}},R=function(e,n){for(var t in e)if(Object.prototype.hasOwnProperty.call(e,t)){var a=e[t];if(Array.isArray(a)){var r,c=Object(S.a)(a);try{for(c.s();!(r=c.n()).done;){var o=r.value;void 0!==o.uid&&n(t,o)}}catch(s){c.e(s)}finally{c.f()}}else a&&void 0!==a.uid&&n(t,a)}},z=h(),B=new Set(["Process","File","IpAddress","Asset","Risk","IpConnections","ProcessInboundConnections","ProcessOutboundConnections"]),U=function(e){var n,t=Object(S.a)(e);try{for(t.s();!(n=t.n()).done;){var a=n.value;a.predicates&&D(a,(function(e,n,t){if(e.predicates&&!B.has(e.predicates.dgraph_type[0])){var a=Object(u.a)({},e.predicates);delete e.predicates,Object.keys(a).forEach((function(n){e[n]=a[n]}))}if(t.predicates&&!B.has(t.predicates.dgraph_type[0])){var r=Object(u.a)({},t.predicates);delete t.predicates,Object.keys(r).forEach((function(e){t[e]=r[e]}))}}))}}catch(r){t.e(r)}finally{t.f()}},W=function(){var e=Object(p.a)(f.a.mark((function e(n){var t,a,r;return f.a.wrap((function(e){for(;;)switch(e.prev=e.next){case 0:return t=H(n),e.next=3,fetch("".concat(z,"graphql"),{method:"post",body:JSON.stringify({query:t}),headers:{"Content-Type":"application/json"},credentials:"include"}).then((function(e){return e.json()})).then((function(e){return console.log("retrieveGraph res",e),e})).then((function(e){return e.data})).then((function(e){return e.lens_scope}));case 3:return a=e.sent,e.next=6,a;case 6:return r=e.sent,console.log("LensWithScope: ",r),U(r.scope),e.abrupt("return",r);case 10:case"end":return e.stop()}}),e)})));return function(n){return e.apply(this,arguments)}}(),H=function(e){return'\n    {\n        lens_scope(lens_name: "'.concat(e,'") {\n            uid,\n            node_key,\n            lens_name,\n            lens_type,\n            dgraph_type,\n            scope {\n                ... on Process {\n                    uid,\n                    node_key, \n                    dgraph_type,\n                    process_name, \n                    process_id,\n                    children {\n                        uid, \n                        node_key, \n                        dgraph_type,\n                        process_name, \n                        process_id,\n                    }, \n                    risks {  \n                        uid,\n                        dgraph_type,\n                        node_key, \n                        analyzer_name, \n                        risk_score\n                    },\n                }\n            \n                ... on Asset {\n                    uid, \n                    node_key, \n                    dgraph_type,\n                    hostname,\n                    asset_ip{\n                        ip_address\n                    }, \n                    asset_processes{\n                        uid, \n                        node_key, \n                        dgraph_type,\n                        process_name, \n                        process_id,\n                    },\n                    files_on_asset{\n                        uid, \n                        node_key, \n                        dgraph_type,\n                        file_path\n                    }, \n                    risks {  \n                        uid,\n                        dgraph_type,\n                        node_key, \n                        analyzer_name, \n                        risk_score\n                    },\n                }\n\n                ... on File {\n                    uid,\n                    node_key, \n                    dgraph_type,\n                    risks {  \n                        uid,\n                        dgraph_type,\n                        node_key, \n                        analyzer_name, \n                        risk_score\n                    },\n                }\n\n                ... on PluginType {\n                    predicates,\n                }\n            }\n        }\n    }\n')},q=function(e,n){var t=!1;return Q(n,(function(a){Object.prototype.hasOwnProperty.call(e,a)||(t=!0,e[a]=n[a])})),t},G=function(e,n){if(!n.nodes&&!n.links)return null;var t,a={nodes:[],links:[]},r=!1,c=new Map,o=new Map,s=Object(S.a)(e.nodes);try{for(s.s();!(t=s.n()).done;){var i=t.value;c.set(i.uid,i)}}catch(k){s.e(k)}finally{s.f()}var l,u=Object(S.a)(n.nodes);try{for(u.s();!(l=u.n()).done;){var d=l.value,f=c.get(d.uid);f?q(f,d)&&(r=!0):(c.set(d.uid,d),r=!0)}}catch(k){u.e(k)}finally{u.f()}var p,m=Object(S.a)(e.links);try{for(m.s();!(p=m.n()).done;){var h=p.value;if(h){var g=h.source.uid||h.source,v=h.target.uid||h.target;o.set(g+h.label+v,h)}}}catch(k){m.e(k)}finally{m.f()}var b,y=Object(S.a)(n.links);try{for(y.s();!(b=y.n()).done;){var j=b.value,E=j.source||j.source,O=j.target||j.target;o.get(E+j.label+O)||(o.set(j.source+j.label+j.target,j),r=!0)}}catch(k){y.e(k)}finally{y.f()}return a.nodes=Array.from(c.values()),a.links=Array.from(o.values()),r?a:null},J=function(e){var n=e.dgraph_type||e.node_type;return n?Array.isArray(n)?n[0]:n:(console.warn("Unable to find type for node ",e),"Unknown")};function V(e,n){return Math.floor(Math.random()*(n-e+1)+e)}var X=function(e){var n=[],t=[],a=new Map;!function e(n,t){T(n,(function(a,r){t(n,a,r),e(r,t)}))}(e,(function(e,n,a){if("scope"!==n){if("Unknown"===J(e))return;if("Unknown"===J(a))return;if("Risk"===J(e))return;if("Risk"===J(a))return;t.push({source:e.uid,label:n,target:a.uid})}})),function e(n,t){t(n),T(n,(function(n,a){e(a,t)}))}(e,(function(e){var n=J(e);if("Unknown"!==n&&"Risk"!==n){var t=function(e,n){console.log("nodetype",e);var t=n;switch(e){case"Process":return t.process_name||t.process_id||"Process";case"Asset":return t.hostname||"Asset";case"File":return t.file_path||"File";case"IpAddress":return t.external_ip||"IpAddress";case"Lens":return t.lens_name||"Lens";default:return e||""}}(n,e),r=Object(u.a)({},e);r.risk=r.risk||0,r.analyzer_names=r.analyzer_names||"";var c,o=Object(S.a)(e.risks||[]);try{for(o.s();!(c=o.n()).done;){var s=c.value;r.risk+=s.risk_score||0,r.analyzer_names+=s.analyzer_name||""}}catch(l){o.e(l)}finally{o.f()}T(e,(function(e,n){r[e]=void 0}));var i=Object(u.a)(Object(u.a)({name:e.uid},r),{},{id:e.uid,nodeType:n,nodeLabel:t,x:200+V(1,50),y:150+V(1,50)});a.set(e.uid,i)}}));var r,c=Object(S.a)(a.values());try{for(c.s();!(r=c.n()).done;){var o=r.value;n.push(o)}}catch(s){c.e(s)}finally{c.f()}return{nodes:n,links:t}},Y=(t(230),function(e,n,t){var a=t<.5?t*(1+n):t+n-t*n,r=2*t-a;return[(e/=360)+1/3,e,e-1/3].map((function(e){return e<0&&e++,e>1&&e--,e=e<1/6?r+6*(a-r)*e:e<.5?a:e<2/3?r+6*(a-r)*(2/3-e):r,Math.round(255*e)}))}),$=function(e){return"[object Array]"===Object.prototype.toString.call(e)},K=function e(n){var t=this;Object(w.a)(this,e),this.hsl=function(e){var n,a,r=t.hash(e);if(t.hueRanges.length){var c=t.hueRanges[r%t.hueRanges.length];n=r/t.hueRanges.length%727*(c.max-c.min)/727+c.min}else n=r%359;return r=parseInt(r/360),a=t.S[r%t.S.length],r=parseInt(r/t.S.length),[n,a,t.L[r%t.L.length]]},this.rgb=function(e){var n=t.hsl(e);return Y.apply(t,n)};var a=[(n=n||{}).lightness,n.saturation].map((function(e){return $(e=e||[.35,.5,.65])?e.concat():[e]}));this.L=a[0],this.S=a[1],"number"===typeof n.hue&&(n.hue={min:n.hue,max:n.hue}),"object"!==typeof n.hue||$(n.hue)||(n.hue=[n.hue]),"undefined"===typeof n.hue&&(n.hue=[]),this.hueRanges=n.hue.map((function(e){return{min:"undefined"===typeof e.min?0:e.min,max:"undefined"===typeof e.max?360:e.max}})),this.hash=n.hash||F},Q=function(e,n){for(var t in e)Object.prototype.hasOwnProperty.call(e,t)&&(Array.isArray(e[t])?e[t].length>0&&void 0===e[t][0].uid&&n(t):n(t))},Z=function(){var e=Object(p.a)(f.a.mark((function e(n,t,a){return f.a.wrap((function(e){for(;;)switch(e.prev=e.next){case 0:if(n){e.next=2;break}return e.abrupt("return");case 2:return e.next=4,W(n).then(function(){var e=Object(p.a)(f.a.mark((function e(r){var c,o;return f.a.wrap((function(e){for(;;)switch(e.prev=e.next){case 0:c=X(r),console.log("update",c),null!==(o=G(t.graphData,c))&&(t.curLensName===n?a(Object(u.a)(Object(u.a)({},t),{},{curLensName:n,graphData:o})):a(Object(u.a)(Object(u.a)({},t),{},{curLensName:n,graphData:c})));case 4:case"end":return e.stop()}}),e)})));return function(n){return e.apply(this,arguments)}}()).catch((function(e){return console.error("Failed to retrieveGraph ",e)}));case 4:case"end":return e.stop()}}),e)})));return function(n,t,a){return e.apply(this,arguments)}}(),ee=function(e){var n=e.lensName,t=e.setCurNode,c=r.a.useState(function(e){return{graphData:{nodes:[],links:[]},curLensName:e}}(n)),o=Object(l.a)(c,2),s=o[0],i=o[1],u=Object(a.useRef)(null);Object(a.useEffect)((function(){u.current.d3Force("link",N.b()),u.current.d3Force("collide",N.a(22)),u.current.d3Force("charge",N.c()),u.current.d3Force("box",(function(){s.graphData.nodes.forEach((function(e){var n=e.x||0,t=e.y||0;Math.abs(n)>1e3&&(e.vx*=-1),Math.abs(t)>1e3&&(e.vy*=-1)}))}))}),[s]),Object(a.useEffect)((function(){Z(n,s,i);var e=setInterval(Object(p.a)(f.a.mark((function e(){return f.a.wrap((function(e){for(;;)switch(e.prev=e.next){case 0:if(!n){e.next=3;break}return e.next=3,Z(n,s,i);case 3:case"end":return e.stop()}}),e)}))),1e3);return function(){return clearInterval(e)}}),[n,s]);var d=s.graphData,m=new K({});return r.a.createElement(r.a.Fragment,null,r.a.createElement(_.a,{graphData:d,nodeLabel:function(e){return e.nodeLabel},enableNodeDrag:!0,linkDirectionalParticles:1,linkDirectionalParticleWidth:function(e){return function(e,n){var t=A(e,n);return t>=75?5:t>=50?4:t>=25?3:2}(e,d)},linkDirectionalParticleColor:function(e){return M(e,d)},linkDirectionalParticleSpeed:.005,onNodeClick:function(e,n){t(e)},linkDirectionalArrowLength:8,linkWidth:4,linkDirectionalArrowRelPos:function(e){return function(e,n){var t=P(e.target.name,n.nodes);if(null===t||0===t.risk)return 1;var a=Object(x.a)(n.nodes).map((function(e){return e.risk})),r=L(t.risk,a);return 0===r?1:r>=75?.95:r>=50?.9:r>=25?.85:1}(e,d)},linkCanvasObjectMode:function(){return"after"},linkCanvasObject:function(e,n){var t=e.source,a=e.target;if(e.color=M(e,d),"object"===typeof t&&"object"===typeof a){var r=Object.assign.apply(Object,Object(x.a)(["x","y"].map((function(e){return Object(k.a)({},e,t[e]+(a[e]-t[e])/2)})))),c={x:a.x-t.x,y:a.y-t.y},o=Math.sqrt(Math.pow(c.x,2)+Math.pow(c.y,2))-96,s=Math.atan2(c.y,c.x);s>Math.PI/2&&(s=-(Math.PI-s)),s<-Math.PI/2&&(s=-(-Math.PI-s));var i=function(e){return"children"===e?"executed":e}(e.label);n.font="50px Arial";var l=Math.min(8,o/n.measureText(i).width);n.font="".concat(l+5,"px Arial");var u=n.measureText(i).width,f=[u+=Math.round(.25*u),l].map((function(e){return e+.2*l}));n.save(),n.translate(r.x,r.y),n.rotate(s),n.fillStyle="rgb(115,222,255,1)",n.fillRect.apply(n,[-f[0]/2,-f[1]/2].concat(Object(x.a)(f))),n.textAlign="center",n.textBaseline="middle",n.fillStyle="white",n.fillText(i,.75,3),n.restore()}},nodeCanvasObject:function(e,n,t){var a=function(e,n){var t=Object(x.a)(n.nodes).map((function(e){return e.risk})),a=L(e.risk,t);return a>=75?6:a>=25?5:4}(e,d);n.save(),n.beginPath(),n.arc(e.x,e.y,1.3*a,0,2*Math.PI,!1),n.fillStyle=function(e,n,t){var a=Object(x.a)(n.nodes).map((function(e){return e.risk})),r=L(e.risk,a);if(0===r){var c=C(e,t);return"rgba(".concat(c[0],", ").concat(c[1],", ").concat(c[2],", 1)")}return I(r)}(e,d,m),n.fill(),n.restore(),n.save(),n.beginPath(),n.arc(e.x,e.y,1.2*a,0,2*Math.PI,!1);var r=C(e,m);n.fillStyle="rgba(".concat(r[0],", ").concat(r[1],", ").concat(r[2],", 1)"),n.fill(),n.restore();var c=e.nodeLabel,o=15/t;n.font="".concat(o,"px Arial");var s=[n.measureText(c).width,o].map((function(e){return e+.2*o}));n.fillStyle="rgba(48, 48, 48, 0.8)",n.fillRect.apply(n,[e.x-s[0]/2,e.y-s[1]/2].concat(Object(x.a)(s))),n.textAlign="center",n.textBaseline="middle",n.fillStyle="rgba(0, 0, 0, 0.8)",n.fillStyle="white",n.fillText(c,e.x,e.y)},ref:u}))},ne=t(265),te=t(272),ae=t(273),re=t(270),ce=t(271),oe=t(267),se=t(269),ie=Object(ne.a)({root:{fontSize:"1em"},table:{minWidth:450},tableHeader:{fontSize:"18px",color:"#EAFDFF"}});var le=function(e){var n=e.node;console.log("Displaying node: ",n);var t=ie(),a=new Set(["id","dgraph.type","__indexColor","risks","uid","scope","name","nodeType","nodeLabel","x","y","index","vy","vx","fx","fy"]);R(n,(function(e,n){a.add(e)}));var c={};return Q(n,(function(e){var t=n[e];a.has(e)||t&&(e.includes("_time")?c[e]=new Date(t).toLocaleString():c[e]=t)})),r.a.createElement(ce.a,null,r.a.createElement(te.a,{className:t.table},function(e,n){return e?r.a.createElement(oe.a,null,r.a.createElement(se.a,null,r.a.createElement(re.a,{align:"left",className:n.tableHeader},r.a.createElement("b",null," PROPERTY ")),r.a.createElement(re.a,{align:"left",className:n.tableHeader},r.a.createElement("b",null," VALUE ")))):r.a.createElement(r.a.Fragment,null)}(n,t),r.a.createElement(ae.a,null,Object.entries(c).map((function(e){var n=Object(l.a)(e,2),t=n[0],a=n[1];return r.a.createElement(se.a,null,r.a.createElement(re.a,{align:"left"},r.a.createElement("b",null,t)),r.a.createElement(re.a,{align:"left"},a))})))))||"no no"},ue=t(274),de=t(97),fe=t.n(de),pe=t(124),me=t.n(pe),he=t(125),ge=t.n(he),ve=t(275),be=Object(ne.a)({root:{fontSize:"1rem"},button:{width:".005%",color:"white",backgroundColor:"#424242"},title:{fontSize:"25px",color:"#ffffff"},icon:{color:"#42C6FF",margin:"15px 0 0 10px"},expand:{color:"#42C6FF",margin:"0px"},header:{display:"flex"},table:{minWidth:450}});function ye(e){return r.a.createElement(r.a.Fragment,null,r.a.createElement(se.a,{key:e.lens},r.a.createElement(re.a,{component:"th",scope:"row"},r.a.createElement(ue.a,{onClick:function(){e.setLens(e.lens)}},e.lens_type+" :\t\t"+e.lens+"\t\t"+e.score))))}function je(e){var n=e.setLens,t=Object(a.useState)({toggled:!0,lenses:[]}),c=Object(l.a)(t,2),o=c[0],s=c[1],i=be();return Object(a.useEffect)((function(){var e=setInterval((function(){console.log("Fetching lenses"),Oe().then((function(e){e.lenses&&e.lenses!==o.lenses&&s(Object(u.a)(Object(u.a)({},o),{},{lenses:e.lenses||[]}))}))}),1e3);return function(){return clearInterval(e)}})),r.a.createElement(r.a.Fragment,null,r.a.createElement("div",{className:i.header},r.a.createElement("b",{className:i.title},r.a.createElement(me.a,{className:i.icon}),"LENSES"),r.a.createElement(ue.a,{className:i.button,onClick:function(){s(Object(u.a)(Object(u.a)({},o),{},{toggled:!o.toggled}))}},r.a.createElement(fe.a,{className:i.expand}))),r.a.createElement("div",{className:"lensToggle"},o.toggled&&o.lenses&&o.lenses.map((function(e){return r.a.createElement(ce.a,null,r.a.createElement(te.a,{className:i.table,"aria-label":"lens table"},r.a.createElement(ae.a,null,r.a.createElement(ye,{key:Number(e.uid),uid:e.uid,lens:e.lens_name,lens_type:e.lens_type,score:e.score,setLens:n}))))}))),r.a.createElement(ve.a,null))}var Ee=h(),Oe=function(){var e=Object(p.a)(f.a.mark((function e(){var n,t;return f.a.wrap((function(e){for(;;)switch(e.prev=e.next){case 0:return console.log("fetching graph from",Ee),"\n    {\n        lenses {\n            uid,\n            node_key,\n            lens_name,\n            score, \n            lens_type,\n        }\n    }\n    ",console.log("connecting to: "+"".concat(Ee,"graphql")),e.next=5,fetch("".concat(Ee,"graphql"),{method:"post",body:JSON.stringify({query:"\n    {\n        lenses {\n            uid,\n            node_key,\n            lens_name,\n            score, \n            lens_type,\n        }\n    }\n    "}),headers:{"Content-Type":"application/json"},credentials:"include"}).then((function(e){return e.json()})).then((function(e){return e.errors&&(console.error("lenses failed",e.errors),e.data={lenses:[]}),e})).then((function(e){return e.data}));case 5:return n=e.sent,e.next=8,n;case 8:return t=e.sent,e.abrupt("return",t);case 10:case"end":return e.stop()}}),e)})));return function(){return e.apply(this,arguments)}}(),ke=function(e){var n=e.node;return r.a.createElement(r.a.Fragment,null,r.a.createElement(le,{node:n}))};function xe(e){var n=e.curNode,t=Object(a.useState)(!0),c=Object(l.a)(t,2),o=c[0],s=c[1],i=be();return r.a.createElement(r.a.Fragment,null,r.a.createElement("div",null,r.a.createElement("div",{className:i.header},r.a.createElement("b",{className:i.title},r.a.createElement(ge.a,{className:i.icon})," NODE"),r.a.createElement(ue.a,{className:i.button,onClick:function(){s((function(e){return!e}))}},r.a.createElement(fe.a,{className:i.expand}))),r.a.createElement("div",{className:"nodeToggle"},o&&n&&r.a.createElement(r.a.Fragment,null,r.a.createElement(ke,{node:n})))))}function we(e){var n=e.setLens,t=e.curNode;return r.a.createElement(r.a.Fragment,null,r.a.createElement(je,{setLens:n}),r.a.createElement(xe,{curNode:t}))}var _e=t(6),Ne=t(283),Se=t(281),Le=t(276),Pe=t(277),Ae=t(279),Fe=t(278),Ce=t(126),Ie=t.n(Ce),Me=Object(ne.a)((function(e){return Object(Ne.a)({root:{display:"flex"},appBar:{transition:e.transitions.create(["margin","width"],{easing:e.transitions.easing.sharp,duration:e.transitions.duration.leavingScreen})},appBarShift:{width:"calc(100% - ".concat(500,"px)"),marginLeft:500,transition:e.transitions.create(["margin","width"],{easing:e.transitions.easing.easeOut,duration:e.transitions.duration.enteringScreen})},menuButton:{marginRight:e.spacing(2),color:"#42C6FF"},hide:{display:"none"},drawer:{width:500,flexShrink:0},drawerPaper:{width:500},drawerHeader:Object(u.a)(Object(u.a)({display:"flex",alignItems:"center",padding:e.spacing(0,1)},e.mixins.toolbar),{},{justifyContent:"flex-end"}),content:{flexGrow:1,padding:e.spacing(3),transition:e.transitions.create("margin",{easing:e.transitions.easing.sharp,duration:e.transitions.duration.leavingScreen}),marginLeft:-500},contentShift:{transition:e.transitions.create("margin",{easing:e.transitions.easing.easeOut,duration:e.transitions.duration.enteringScreen}),marginLeft:0},lensName:{color:"#EAFDFF",fontSize:"2rem",margin:"10px 15px 0px 0px"},header:{fontSize:"35px"},close:{color:"#42C6FF"}})}));function De(e){var n=e.setLens,t=e.curLens,a=e.curNode,c=Me(),o=r.a.useState(!0),s=Object(l.a)(o,2),i=s[0],u=s[1];return r.a.createElement("div",{className:c.root},r.a.createElement(Le.a,{position:"fixed",className:Object(_e.a)(c.appBar,Object(k.a)({},c.appBarShift,i))},r.a.createElement(Pe.a,null,r.a.createElement(Fe.a,{color:"inherit","aria-label":"open drawer",onClick:function(){u(!0)},edge:"start",className:Object(_e.a)(c.menuButton,i&&c.hide)},"\u2630"),r.a.createElement(Ae.a,{variant:"h5",noWrap:!0},r.a.createElement("b",{className:c.header}," GRAPL")))),r.a.createElement(Se.a,{className:c.drawer,variant:"persistent",anchor:"left",open:i,classes:{paper:c.drawerPaper}},r.a.createElement("div",{className:c.drawerHeader},r.a.createElement(ue.a,{onClick:function(){u(!1)}},r.a.createElement(Ie.a,{className:c.close}))),r.a.createElement(ve.a,null),r.a.createElement(we,{setLens:n,curNode:a})),r.a.createElement("main",{className:Object(_e.a)(c.content,Object(k.a)({},c.contentShift,i))},r.a.createElement("div",{className:c.drawerHeader}),r.a.createElement("h3",{className:c.lensName},t||""),r.a.createElement(Ae.a,{paragraph:!0})))}var Te=function(){var e=r.a.useState({curLens:"",curNode:null}),n=Object(l.a)(e,2),t=n[0],a=n[1];return console.log("EngagementUX: curLens, ",t.curLens),r.a.createElement(r.a.Fragment,null,r.a.createElement(De,{setLens:function(e){return a(Object(u.a)(Object(u.a)({},t),{},{curLens:e}))},curLens:t.curLens,curNode:t.curNode}),r.a.createElement(ee,{lensName:t.curLens,setCurNode:function(e){a(Object(u.a)(Object(u.a)({},t),{},{curNode:e}))}}))};console.log("App loading");var Re=function(e,n,t){n(Object(u.a)(Object(u.a)({},e),{},{curPage:t})),localStorage.setItem("grapl_curPage",t)},ze=function(){var e=r.a.useState({curPage:localStorage.getItem("grapl_curPage")||"login",lastCheckLoginCheck:Date.now()}),n=Object(l.a)(e,2),t=n[0],c=n[1];return Object(a.useEffect)((function(){"login"!==t.curPage&&Date.now()-t.lastCheckLoginCheck>1e3&&v().then((function(e){console.log("Not logged in, redirecting."),e||"login"===t.curPage||Re(t,c,"login")}))})),"login"===t.curPage?(console.log("routing to engagement_ux page"),r.a.createElement(b,{loginSuccess:function(){return Re(t,c,"engagement_ux")}})):"engagement_ux"===t.curPage?(console.log("routing to login page"),r.a.createElement(Te,null)):(console.warn("Invalid Page State"),r.a.createElement("div",null,"Invalid Page State"))};function Be(){return console.log("App loaded"),r.a.createElement(r.a.Fragment,null,r.a.createElement(ze,null))}Boolean("localhost"===window.location.hostname||"[::1]"===window.location.hostname||window.location.hostname.match(/^127(?:\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)){3}$/));var Ue=Object(s.a)({palette:{type:"dark",primary:{main:"#373740"}}}),We=document.getElementById("root");o.a.render(r.a.createElement(r.a.StrictMode,null,r.a.createElement(i.a,{theme:Ue},r.a.createElement(Be,null),",")),We),"serviceWorker"in navigator&&navigator.serviceWorker.ready.then((function(e){e.unregister()})).catch((function(e){console.error(e.message)}))}},[[144,1,2]]]);
//# sourceMappingURL=main.820d66df.chunk.js.map