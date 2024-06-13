export default class MarqueeState {
	overflow = $state(false);
	marqueeScrollWidth = $state(0);
	animating = $state(false);
	reseted = $state(false);
	ismodifing = false;

	updateOverflow(content: HTMLDivElement, marquee: HTMLDivElement) {
		if (!this.ismodifing) {
			this.ismodifing = true;
			this.overflow =
				(this.overflow ? content.clientWidth / 2 : content.clientWidth) >= marquee.clientWidth;
			if (this.overflow) {
				if (!this.reseted) {
					this.animating = true;
				}
			}
			this.marqueeScrollWidth = content.scrollWidth;
			this.ismodifing = false;
		}
	}

	reset() {
		this.animating = false;
		this.reseted = true;
		setTimeout(() => {
			this.reseted = false;
			this.animating = true;
		}, 5000);
	}
}
