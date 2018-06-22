import React, { Component } from 'react';
import PropTypes from 'prop-types';

class DropdownButton extends Component {
	constructor(props) {
		super(props);
		this.state = {
			show: false
		}

		this.toggleDropdown = this.toggleDropdown.bind(this);
		this.setWrapperRef = this.setWrapperRef.bind(this);
	}
	 /**
	 * Set the wrapper ref
	 */
	setWrapperRef(node) {
		this.wrapperRef = node;
	}
	componentDidMount() {
		document.addEventListener('click', this.closeDropdown.bind(this));
	}
	componentWillUnmount() {
		document.removeEventListener('click', this.closeDropdown);
	}
	toggleDropdown() {
		let state = {...this.state, show: !this.state.show};
		this.setState(state);
	}
	closeDropdown(event) {
		if (this.wrapperRef && !this.wrapperRef.contains(event.target)) {
			let state = {...this.state, show: false};
			this.setState(state);
		}
	}
	render() {
		let activeClass = this.state.show ? 'is-active' : '';
		return (
			<div ref={this.setWrapperRef} className={"dropdown is-right "+activeClass} onClick={this.toggleDropdown}>
				<div className="dropdown-trigger">
					<button className="button" aria-haspopup="true" aria-controls="dropdown-menu">
						<span>{this.props.label}</span>
						<span className="icon is-small">
							<i className="fas fa-angle-down" aria-hidden="true"></i>
						</span>
					</button>
				</div>
				<div className="dropdown-menu" id="dropdown-menu" role="menu">
					{this.props.children}
				</div>
			</div>
		);
	}
}

DropdownButton.defaultProps = {
	
}

DropdownButton.propTypes = {
	label: PropTypes.string.isRequired,
}

export default DropdownButton;