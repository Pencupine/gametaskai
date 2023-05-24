export const rowContainerStyle = {
    display: 'flex',
    flexDirection: 'row',
    justifyContent: 'flex-end',
    alignItems: 'flex-end'
};

export function leftColumnStyle (leftColumnWidth) {
    return {
        flex: `${leftColumnWidth}%`
    }
}

export function rightColumnStyle (rightColumnWidth){
    return {
        flex: `${100-rightColumnWidth}%`
    }
}

export const sliderStyle = {marginLeft: 'auto', innerHeight: 200}